# Nucleo Test (STM32F446ZE)

A minimal Rust project for the STM32F446ZE Nucleo board using `probe-rs` and
`stm32f4xx-hal`.

## Setup (Debian Trixie)

1. Clone the Repository
    ```
    git clone git@github.com:haba713/nucleo_test.git
    cd nucleo_test/
    ```
2. Install Required Tools

    - Rust toolchain: https://www.rust-lang.org/tools/install
    - probe-rs CLI tools: https://probe.rs/docs/getting-started/installation/
    - ARM toolchain & dependencies:
        ```
        sudo apt update
        sudo apt install -y gcc-arm-none-eabi libudev-dev pkg-config build-essential
        ```
        
3. Add Rust Target for ARM Cortex-M
    ```
    rustup target add thumbv7em-none-eabihf
    ```
    
4. Build & Flash Firmware
    ```
    cargo build --release
    cargo run --release
    ```
    This uses `probe-rs` to flash and run the binary on your STM32F446ZE board
    via SWD.

## Project Files

- [src/main.rs]: Main firmware source
- [memory.x]: Memory layout for STM32F446ZE (512K Flash, 128K RAM)
- [build.rs]: Ensures `memory.x` is linked during build
- [.cargo/config.toml]: Sets the target and probe-rs runner

## Notes

- You may need to set up udev rules to allow non-root USB access for ST-Link.
- This project uses `thumbv7em-none-eabihf` target (Cortex-M4 with FPU).
- `probe-rs` uses SWD protocol with ST-Link for flashing and running.
