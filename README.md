# Introduction

This repository is for reporting a bug with the `stm32f030k6` feature
of [embassy](https://github.com/embassy-rs).

# Development

Ensure [Rust is installed](https://rustup.rs/) and updated to the latest version.

Install the MCU target:

`rustup target add thumbv6m-none-eabi`

Install probe-rs CLI tools

`cargo install probe-rs --features cli`

Connect the device over USB and run

`cargo run`
