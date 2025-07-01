# Nucleo F446ZE Test

A minimal Rust project for the Nucleo F446ZE board with the STM32F446ZE
microcontroller using `probe-rs` and `stm32f4xx-hal`. The application
sequentially lights three user LEDs (L01 green on PB0, L02 blue on PB7, L03 red
on PB14) with a 200 ms delay per step in the order L01, L02, L03, L02, L01, and
so on.

## Setup (Debian Trixie)

1. Clone the repository:
    ```
    git clone git@github.com:haba713/nucleo_test.git
    cd nucleo_test/
    ```
    
2. Install required tools:

    - Rust toolchain: https://www.rust-lang.org/tools/install
    - probe-rs CLI tools: https://probe.rs/docs/getting-started/installation/
    - ARM toolchain & dependencies:
        ```
        sudo apt update
        sudo apt install -y gcc-arm-none-eabi libudev-dev pkg-config build-essential
        ```
        
3. Add Rust target for ARM Cortex-M:
    ```
    rustup target add thumbv7em-none-eabihf
    ```
    
4. Build & flash the application:
    ```
    cargo run --release
    ```

    This uses `probe-rs` to flash and run the binary on your STM32F446ZE board
    via SWD.

5. To stop the application, press Ctrl+C in the terminal and then run:
    ```
    probe-rs reset --chip STM32F446ZE --protocol swd
    ```

## Project Files

- [main.rs](src/main.rs): Main application source controlling LEDs
- [memory.x](memory.x): Memory layout for STM32F446ZE (512K Flash, 128K RAM)
- [build.rs](build.rs): Ensures `memory.x` is linked during build
- [config.toml](.cargo/config.toml): Sets the target and probe-rs runner

## Notes

- This project uses `thumbv7em-none-eabihf` target (Cortex-M4 with FPU).
- `probe-rs` uses SWD protocol with ST-Link for flashing and running.
