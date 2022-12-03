# Maker Advent Calendar 2022 Python And Rust

This repository contains the source code for the Maker Advent Calendar 2022. The main objectif of this project is to reproduce the Python code in rust.

[The PiHut advent](https://thepihut.com/pages/advent)

[Pico Pinout](https://cdn.shopify.com/s/files/1/0176/3274/files/Pico-R3-A4-Pinout_f22e6644-b3e4-4997-a192-961c55fc8cae.pdf?v=1664490511)

## How to use

### Reset the board

1. Disconnect the board
2. Press the button near the usb port
3. Plug the board
4. Congratulation, the board is reset

### Burn the python firmware

1. Ensure that the board is reset
2. Download the firmware from [here](https://micropython.org/)
3. put the .uf2 file in the drive that seems to be your board (it might contains two file)
4. Enjoy!

### Burn the rust firmware

In this case, you will have different firmware each time you want to burn your project.

1. Ensure that the board is reset
2. Build and run your rust project `cargo build --release && cargo run --release`
3. Copy paste the .uf2 file (mostly in ./target/thumbv6m-none-eabi/release/) in the drive that seems to be your board (it might contains two file)

## Dev

### 1. Main needed tools

- Vscode

### 2. Python

#### 2.1 Needed

- [Python](https://www.python.org/downloads/)
- [PyMakr vscode extension](https://marketplace.visualstudio.com/items?itemName=pycom.Pymakr)

#### 2.2 How to use

1. Create a project with the PyMakr extension
2. write some code
3. Connect to the board through the PyMakr extension
4. Push your code to the board

### 3. Rust

This project work with the [Rp-hal rust](https://github.com/rp-rs/rp-hal/tree/main/boards/rp-pico) crate. It provide almost everything you need to work with the pico board.

#### 3.1 Needed

- [Rust](https://www.rust-lang.org/tools/install)
- [Rust-analyzer vscode extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
