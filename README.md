<p align="center"><img width="150" src="pff.png" alt="pff logo"></p>

# `pff`
Examine your Internet connection quality and status in terminal

[![asciicast](https://asciinema.org/a/v56HwMPcn9TXWQgP8f4exM1OG.svg)](https://asciinema.org/a/v56HwMPcn9TXWQgP8f4exM1OG)

## Usage
Download pff from [releases](https://github.com/jmaczan/pff/releases)

To use it globally, copy pff to `/usr/bin`

You might need to run `sudo setcap cap_net_raw+ep <path to pff>` to give it rights to `CAP_NET_RAW`

## Build
If you don't want to use a released binary version, build pff on your own. [Clone](https://github.com/jmaczan/pff.git) this repository

Run `cargo build --release` in a project root directory. You need Rust and Cargo to build it

A binary file will be available in `target/release/pff`

## Cite
If you use this software in your research, please use the following citation:

```bibtex
@misc{Maczan_pff_2024,
  title = "Modern ping alternative for macOS and Linux",
  author = "{Maczan, Jędrzej Paweł}",
  howpublished = "\url{https://github.com/jmaczan/pff}",
  year = 2022,
  publisher = {GitHub}
}
```

## License
GNU GPL v2

Copyright [Jędrzej Paweł Maczan](https://maczan.pl/), 2022
