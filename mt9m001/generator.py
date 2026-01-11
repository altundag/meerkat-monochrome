from argparse import ArgumentParser
from dataclasses import dataclass, field
from pathlib import Path
import re
import json

LIB = """// This file was automatically generated from {file}\n\n#![no_std]

use embedded_hal::i2c::I2c;"""

CONST = "const {name}: u8 = {address};"

BIT_FIELD_STRUCT = """pub struct {name} {{
    value: u16,
}}

impl {name} {{
    pub const DEFAULT: Self = Self::new({default});

    pub const fn new(value: u16) -> Self {{
        Self {{ value }}
    }}

    {functions}
}}"""


SENSOR_STRUCT = """pub struct {name}<I2C>
where
    I2C: I2c,
{{
    i2c: I2C,
}}

impl<I2C> {name}<I2C>
where
    I2C: I2c,
{{
    pub const fn new(i2c: I2C) -> Self {{
        Self {{ i2c }}
    }}

    fn get_u16(&mut self, reg: u8) -> Result<u16, I2C::Error> {{
        self.i2c.write(SENSOR_ADDRESS, &[reg])?;
        let mut bytes = [0; 2];
        self.i2c.read(SENSOR_ADDRESS, &mut bytes)?;

        Ok(u16::from_be_bytes(bytes))
    }}

    fn set_u16(&mut self, reg: u8, value: u16) -> Result<(), I2C::Error> {{
        let bytes = value.to_be_bytes();
        self.i2c.write(SENSOR_ADDRESS, &[reg, bytes[0], bytes[1]])?;
        Ok(())
    }}

    {functions}
}}"""

GETTER = """/// {documentation}
pub fn {name}(&mut self) -> Result<u16, I2C::Error> {{
    self.get_u16({address})
}}"""

SETTER = """/// {documentation}
pub fn {name}(&mut self, value: u16) -> Result<(), I2C::Error> {{
    self.set_u16({address}, value)
}}"""

BIT_FIELD_STRUCT_GETTER = """/// {documentation}
pub fn {name}(&mut self) -> Result<{struct_name}, I2C::Error> {{
    Ok({struct_name}::new(self.get_u16({address})?))
}}"""

BIT_FIELD_STRUCT_SETTER = """/// {documentation}
pub fn {name}(&mut self, value: &{struct_name}) -> Result<(), I2C::Error> {{
    self.set_u16({address}, value.value)
}}"""


BIT_RANGE_GETTER = """/// {documentation}
pub const fn {name}(&self) -> u16 {{
    self.value >> {start} & !(0xFFFFu16 << ({end} - {start}))
}}"""

BIT_RANGE_SETTER = """/// {documentation}
pub const fn {name}(mut self, value: u16) -> Self {{
    let mask = (!0u16 << {end}) | ((1u16 << {start}) - 1);
    self.value = (self.value & mask) | ((value << {start}) & !mask);

    self
}}"""


BIT_GETTER = """/// {documentation}
pub const fn {name}(&self) -> bool {{
        (self.value & (1u16 << {bit_position})) != 0
}}"""

BIT_SETTER = """/// {documentation}
pub const fn {name}(mut self, value: bool) -> Self {{
    self.value = (self.value & !(1u16 << {bit_position})) | ((value as u16) << {bit_position});
    self
}}"""


@dataclass
class FieldOrFunction:
    name: str
    documentation: list[str]

    def sneak_case(self) -> str:
        s = re.sub(r"[^a-zA-Z0-9]+", "_", self.name).strip("_").lower()
        return f"_{s}" if s and s[0].isdigit() else s

    def pascal_case(self) -> str:
        parts = re.findall(r"[a-zA-Z0-9]+", self.name)
        pascal = "".join(word.capitalize() for word in parts)
        return f"_{pascal}" if pascal and pascal[0].isdigit() else pascal

    def doc(self):
        lines = "\n/// ".join(self.documentation)
        return f"{lines}"


@dataclass
class Bit(FieldOrFunction):
    bit_position: str


@dataclass
class Register(FieldOrFunction):
    address: str
    default: str | None = None
    bits: list[Bit] = field(default_factory=list)

    def render_const(self) -> str:
        return CONST.format(name=self.sneak_case().upper(), address=self.address)

    def render_struct(self) -> str:
        if self.default is None:
            raise ValueError(
                f"cannot render {self.name} because it doesn't have a default value"
            )
        functions = []
        for bit in self.bits:
            name = bit.sneak_case()
            documentation = bit.doc()
            if ":" in bit.bit_position:
                end, start = bit.bit_position.split(":")
                end = int(end) + 1
                setter = BIT_RANGE_SETTER.format(
                    name=f"set_{name}",
                    start=start,
                    end=end,
                    documentation=documentation,
                )
                getter = BIT_RANGE_GETTER.format(
                    name=f"get_{name}",
                    start=start,
                    end=end,
                    documentation=documentation,
                )
            else:
                setter = BIT_SETTER.format(
                    name=f"set_{name}",
                    bit_position=bit.bit_position,
                    documentation=documentation,
                )
                getter = BIT_GETTER.format(
                    name=f"get_{name}",
                    bit_position=bit.bit_position,
                    documentation=documentation,
                )
            functions.append(setter)
            functions.append(getter)
        return BIT_FIELD_STRUCT.format(
            name=self.pascal_case(),
            default=self.default,
            functions="\n\n".join(functions),
        )

    def render_getter(self) -> str:
        return GETTER.format(
            documentation=self.doc(),
            name=f"get_{self.sneak_case()}",
            address=self.sneak_case().upper(),
        )

    def render_setter(self) -> str:
        return SETTER.format(
            documentation=self.doc(),
            name=f"set_{self.sneak_case()}",
            address=self.sneak_case().upper(),
        )

    def bit_field_struct_getter(self) -> str:
        return BIT_FIELD_STRUCT_GETTER.format(
            documentation=self.doc(),
            name=f"get_{self.sneak_case()}",
            address=self.sneak_case().upper(),
            struct_name=self.pascal_case(),
        )

    def bit_field_struct_setter(self) -> str:
        return BIT_FIELD_STRUCT_SETTER.format(
            documentation=self.doc(),
            name=f"set_{self.sneak_case()}",
            address=self.sneak_case().upper(),
            struct_name=self.pascal_case(),
        )


if __name__ == "__main__":
    argparser = ArgumentParser()
    argparser.add_argument("input", type=Path)
    argparser.add_argument("output", type=Path)
    args = argparser.parse_args()

    with open(args.input) as fh:
        sensor = json.load(fh)
        description = sensor["description"]
        registers: list[Register] = []
        for register in sensor["registers"]:
            register = Register(**register)
            bits = []
            for bit in register.bits:
                bits.append(Bit(**bit))
            register.bits = bits
            registers.append(register)

    with open(args.output, "w") as fh:
        # Begin lib.rs
        fh.write(LIB.format(file=args.input.name))
        fh.write("\n\n")

        # Write sensor address constant
        fh.write(CONST.format(name="SENSOR_ADDRESS", address=description["address"]))
        fh.write("\n")

        # Write register address constants
        for register in registers:
            fh.write(register.render_const())
            fh.write("\n")
        fh.write("\n")

        # Write bit filed structs
        for register in registers:
            if len(register.bits) == 0:
                continue
            fh.write(register.render_struct())
            fh.write("\n\n")
        fh.write("\n")

        # Write bit or value getters/setters
        functions = []
        for register in registers:
            if len(register.bits) > 0:
                functions.append(register.bit_field_struct_getter())
                functions.append(register.bit_field_struct_setter())
            else:
                functions.append(register.render_getter())
                functions.append(register.render_setter())

        # Wrtie main sensor struct
        fh.write(
            SENSOR_STRUCT.format(
                name=description["name"].upper(), functions="\n\n".join(functions)
            )
        )
        fh.write("\n")
