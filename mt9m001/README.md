# MT9M001C12STM

An automatically generated Rust library to control MT9M001C12STM image sensors.

## Generation

Run the following command to generate the library from the JSON description file.

```shell
python .\generator.py .\mt9m001.json .\src\lib.rs
```

The generator implementation is primitive and may produce code with minor `clippy` issues. Run the following command to automatically fix them.

```shell
cargo clippy --fix --allow-dirty
```

## Sensor Description File

The expected sensor description file is a simple JSON file that:
- contains the sensor name and address
- list the registers, their documentation, and their bit-fields (if any)

See the `mt9m001.json` file for an examples.
