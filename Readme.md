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

#### 2.2 BootStrap

1. Create a project with the PyMakr extension
2. write some code
3. Connect to the board through the PyMakr extension
4. Push your code to the board

### 3. Rust

This project work with the [Rp-hal rust](https://github.com/rp-rs/rp-hal/tree/main/boards/rp-pico) crate. It provide almost everything you need to work with the pico board.

/!\ Starting from day 4, the project include the serial workflow. You will need a serial terminal like [Putty](https://www.putty.org/) to see the output of the program.

#### 3.1 Needed

- [Rust](https://www.rust-lang.org/tools/install)
- [Rust-analyzer vscode extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

#### 3.2 BootStrap

You can either start with a blank project or by using one of the example folder like `day2-rust`.

If you want to start with a blank project, you can do the following:

1. Create a new project `cargo init` in a blank folder.
2. Add the following to your `Cargo.toml` file under dependencies:

```toml
[dependencies]
rp-pico = "0.5.0"
cortex-m = "0.7.2"
embedded-hal = "0.2.7"
cortex-m-rt = "0.7.2"
panic-halt = "0.2.0"
```

3. Add the configuration to your .cargo/config.toml file (you can copy paste the one from the example folder). This will allow you to build and package your project in a .uf2 file.

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "elf2uf2-rs"
rustflags = [
"-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv6m-none-eabi"

[env]
DEFMT_LOG = "debug"
```

4. You can use the following code in your main.rs:

```rust
#![no_std]
#![no_main]

use panic_halt as _;
use rp_pico::entry;

#[entry]
fn main() -> ! {
}

```

5. Enjoy!
