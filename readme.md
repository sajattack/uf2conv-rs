# uf2conv

Adds a uf2 header [Microsofts HID Flashing Format (UF2)](https://github.com/microsoft/uf2/blob/86e101e3a282553756161fe12206c7a609975e70/README.md) for copying over to UF2 bootloader mass storage devices. UF2 is factory programmed extensively by [Microsoft MakeCode](https://www.microsoft.com/en-us/makecode) and [Adafruit](https://www.adafruit.com/) hardware.

## Install
`cargo install uf2conv`

## Usage
```bash
 $ uf2conv
error: The following required arguments were not provided:
    <INPUT>

USAGE:
    uf2conv <INPUT> --base <base> --family <family> --output <output>

For more information try --help
```

Base usage you'll want to give the --base memory address for your chip. This is where the code starts after the bootloader. In the case of embedded rust, thats found in your memory.x where you'll find  `FLASH (rx) : ORIGIN = 0x00000000+0x4000` or `FLASH (rx) : ORIGIN = 0x00000000 + 16K` Where 16K in bytes is 16384 decimal or 0x4000 hex base.

```bash
$ uf2conv pygamer_blinky_basic.bin --base 0x4000 --output pygamer_blinky_basic.uf2
```

And you can copy that uf2 file to your embedded device's drive that appears when you enter bootloader mode.

## Rust: How to get a bin file

Use [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) which replaces the `cargo build` command to find and convert elf files into binary. 

Install the dependencies
```bash
$ rustup component add llvm-tools-preview
$ cargo install uf2conv cargo-binutils
```

Then in your embedded project, say [PyGamer](https://github.com/atsamd-rs/atsamd/tree/master/boards/pygamer)
```bash
$ cargo objcopy --example blinky_basic --features unproven --release -- -O binary pygamer_blinky_basic.bin
$ uf2conv pygamer_blinky_basic.bin --base 0x4000 --output pygamer_blinky_basic.uf2
```
