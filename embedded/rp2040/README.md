# Workspace containing all crates targeting the rp2040 microcontroller

## Prerequisites

In order to build the crates within this workspace you need to install the target for the rp2040:

```sh
rustup target add thumbv6m-none-eabi
sudo apt install libudev-dev
cargo install elf2uf2-rs
```
