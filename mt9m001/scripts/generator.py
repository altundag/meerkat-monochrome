import re

REGISTER_TABLE = """0x00 Chip Version 1000 0100 0001 0001 0x8431
0x01 Row Start 0000 0ddd dddd dddd 0x000C
0x02 Column Start 0000 0ddd dddd dddd 0x0014
0x03 Row Size (Window Height) 0000 0ddd dddd dddd 0x03FF
0x04 Col Size (Window Width) 0000 0ddd dddd dddd 0x04FF
0x05 Horizontal Blanking 0000 0ddd dddd dddd 0x0009
0x06 Vertical Blanking 0000 0ddd dddd dddd 0x0019
0x07 Output Control 0000 0000 0d00 00dd 0x0002
0x09 Shutter Width 00dd dddd dddd dddd 0x0419
0x0B Restart 0000 0000 0000 000d 0x0000
0x0C Shutter Delay 0000 0ddd dddd dddd 0x0000
0x0D Reset 0000 0000 0000 000d 0x0000
0x1E Read Options 1 1000 dddd 00dd dd00 0x8000
0x20 Read Options 2 dd01 0dd1 d00d d10d 0x1104
0x2B Even Row, Even Column 0000 0000 0ddd dddd 0x0008
0x2C Odd Row, Even Column 0000 0000 0ddd dddd 0x0008
0x2D Even Row, Odd Column 0000 0000 0ddd dddd 0x0008
0x2E Odd Row, Odd Column 0000 0000 0ddd dddd 0x0008
0x35 Global Gain 0000 0000 0ddd dddd 0x0008
0x5F Cal Threshold dddd dddd d0dd dddd 0x0904
0x60 Even Row, Even Column 0000 000d dddd dddd 0x0000
0x61 Odd Row, Odd Column 0000 000d dddd dddd 0x0000
0x62 Cal Ctrl d00d d100 1001 1ddd 0x0498
0x63 Even Row, Odd Column 0000 000d dddd dddd 0x0000
0x64 Odd Row, Even Column 0000 000d dddd dddd 0x0000
0xF1 Chip Enable 0000 0000 0000 00dd 0x0001
"""

REGEX = r"(0x[0-9a-fA-F]+)\s(.*?)\s([01d]{4}\s[01d]{4}\s[01d]{4}\s[01d]{4})\s(0x[0-9a-fA-F]+)\n"

ENUM_NAME_MAP = {
    "0x03": "Row Size",
    "0x04": "Col Size",
    "0x2B": "Gain Even Row, Even Column",
    "0x2C": "Gain Odd Row, Even Column",
    "0x2D": "Gain Even Row, Odd Column",
    "0x2E": "Gain Odd Row, Odd Column",
    "0x60": "Analog Offset Correction Even Row, Even Column",
    "0x61": "Analog Offset Correction Odd Row, Odd Column",
    "0x63": "Analog Offset Correction Even Row, Odd Column",
    "0x64": "Analog Offset Correction Odd Row, Even Column",
}

CONST_VAR_TEMPLATE = """
///{description}, ({bit_format})
pub(crate) const {name}: u8 = {register};
"""

GET_FUNC_TEMPLATE = """/// Returns {description} value ({bit_format})
pub fn {function_name}(&mut self) -> Result<u16, Error> {{
    self.get_u16(registers::{register_name})
}}

"""

SET_FUNC_TEMPLATE = """/// Sets {description} value ({bit_format})
pub fn {function_name}(&mut self, value: u16) -> Result<(), Error> {{
    self.set_u16(registers::{register_name}, value)
}}

"""

registers = []
for match in re.finditer(REGEX, REGISTER_TABLE, re.MULTILINE):
    registers.append(match.groups())

with open("registers.rs", "w") as fh:
    fh.write("/// Automatically generated\n")
    for entry in registers:
        register, desc, format, default = entry
        name = (
            ENUM_NAME_MAP.get(register, desc).replace(" ", "_").replace(",", "").upper()
        )
        fh.write(
            CONST_VAR_TEMPLATE.format(
                description=desc.capitalize(),
                bit_format=format,
                name=name,
                register=register,
            )
        )

with open("get_set.rs", "w") as fh:
    fh.write("/*\n")
    for entry in registers:
        register, desc, format, default = entry
        function_name = (
            ENUM_NAME_MAP.get(register, desc).replace(" ", "_").replace(",", "").lower()
        )
        register_name = (
            ENUM_NAME_MAP.get(register, desc).replace(" ", "_").replace(",", "").upper()
        )
        fh.write(
            GET_FUNC_TEMPLATE.format(
                description=desc.capitalize(),
                bit_format=format,
                function_name=f"get_{function_name}",
                register_name=register_name,
            )
        )
        fh.write(
            SET_FUNC_TEMPLATE.format(
                description=desc.capitalize(),
                bit_format=format,
                function_name=f"set_{function_name}",
                register_name=register_name,
            )
        )

    fh.write("*/\n")
