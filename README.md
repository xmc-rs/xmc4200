# xmc4200

> THIS IS A WORK IN PROGRESS AND MUCH IS UNTESTED

[![crates.io](https://img.shields.io/crates/v/xmc4200.svg)](https://crates.io/crates/xmc4200)
[![rust](https://github.com/xmc-rs/xmc4200/workflows/Rust/badge.svg)](https://github.com/xmc-rs/xmc4200/workflows/Rust/badge.svg)

This is a 'peripheral access crate' for interfacing to the XMC4200 series of microcontrollers for embedded support in Rust that is generated using [svd2rust](https://docs.rs/svd2rust) and an SVD file provided by Infineon.

There is an optional `rt` that can be utilized. An explanation of the feature is given by [svd2rust](https://docs.rs/svd2rust/0.16.1/svd2rust/#the-rt-feature)

All API's and usage (besides what registers exist) are defined by [svd2rust](https://docs.rs/svd2rust)

## Generate Crate from SVD

```bash
# Necessary 3rd-party tools
cargo install svd2rust
cargo install form
rustup component add rustfmt

svd.sh # Generates code from crate and formats to rustfmt
```

## Using Crate

```toml
[dependencies.xmc4200]
version = "0.2.1"
features = ["rt"]
```
