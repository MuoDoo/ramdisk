# RamDisk

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Description

A simple tool to make a ramdisk in Linux.This can make a specific directory in your system to be a ramdisk, which is a disk that is stored in your RAM. This can be useful for applications that require a lot of read/write operations, such as databases or Machine Learning datasets.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Installation

```bash
git clone https://github.com/MuoDoo/ramdisk.git
cd ramdisk
cargo build --release
```
The binary will be in `target/release/ramdisk`
## Usage
```
Usage: ramdisk <COMMAND>

Commands:
  mount, -m, --mount      Mount a ramdisk
  unmount, -u, --unmount  Unmount a ramdisk
  help                    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
## License

This project is licensed under the [MIT License](LICENSE).

## Contact

If you have any questions or feedback, feel free to reach out to me at [muodoo@icloud.com](mailto:muodoo@icloud.com) or open an issue in the GitHub repository.

