# qmk-hid

[![Build/Lint](https://github.com/luckasRanarison/qmk-hid/actions/workflows/ci.yml/badge.svg)](https://github.com/luckasRanarison/qmk-hid/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/qmk-hid)](https://crates.io/crates/qmk-hid)

A simple CLI for bidirectional communication with [QMK](https://qmk.fm/) keyboards using [raw HID](https://docs.qmk.fm/features/rawhid), written in [Rust](https://www.rust-lang.org/) with a small memory footprint.

## Installation

You can install qmk-hid using [Cargo](https://doc.rust-lang.org/cargo/) or by [building](#build) it from source.

```sh
cargo install qmk-hid
```

## Usage

To use qmk-hid you must find you keyboard's vendor ID and product ID. They can be usually found in your keyboard's `info.json` or using tools like [lsusb](https://wiki.debian.org/lsusb).

On Linux, you have to setup [udev rules](https://wiki.debian.org/udev) to allow access to the HID device.

```sh
qmk-hid --help ## prints help
qmk-hid --vendor-id 18003 --product-id 4 ## example using a corve v4 keyboard
```

Once `qmk-hid` is running:
- Incoming HID messages from the keyboard will be printed to standard output.
- Messages can be sent to the keyboard by writing to standard input.
- Errors and connection issues are reported to standard error output.
- If the keyboard is not connected, `qmk-hid` will continuously poll for a connection until the device is available (the default interval is 3000ms).

> [!IMPORTANT]
> In both directions the message length should be 32 bytes or less, you have to implement a custom protocol overwise

Since qmk-hid uses standard input and output for communication, it can be easily integrated into scripts or other programs. You can checkout the [NodeJS demo](./demo/host.js) for a simple example.

## Build

To build qmk-hid you will need the Rust [toolchain](https://rustup.rs/) and [libhidapi](https://github.com/libusb/hidapi) system libraries. Then you can just clone the repository and use the following command:

```sh
cargo build --release
```
