# Meerkat Firmware

## Build and Run

To build and run the firmware, first install Rust, then add the `thumbv8m.main-none-eabihf` target:

```shell
rustup target add thumbv8m.main-none-eabihf
```

NExt, make the [picotool executable](https://github.com/raspberrypi/pico-sdk-tools/releases) available in your system's environment path. Once that's done, run:

```shell
cargo run --release
```
