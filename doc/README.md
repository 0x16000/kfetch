# kfetch

**kfetch** is a system information tool written in Rust. It displays your system details in a ASCII art layout, with color highlighting for the distribution.

## Features

- Detects Linux distribution and shows corresponding ASCII art.
- Displays system information:
  - Username and hostname
  - Operating system
  - Kernel version
  - Uptime
  - Shell
  - Window manager (WM)
  - Memory usage

## Supported Distributions

- CachyOS
- NixOS
- Arch Linux
- Arco Linux
- Artix Linux
- CentOS
- Debian
- EndeavourOS
- ElementaryOS
- Fedora
- Gentoo
- Linux Mint
- Manjaro
- OpenSUSE
- Slackware
- Ubuntu
- Void Linux
- Raspbian

*(Other distributions will default to a generic ASCII art such as TUX.)*

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository:

```bash
git clone https://github.com/0x16000/kfetch.git
cd kfetch
```

3. Build the project:

```bash
cargo build --release
```

4. Run the tool:

```bash
cargo run
```

Or, run the compiled binary from `target/release/kfetch`.

## Usage

Simply execute kfetch. It will automatically detect your Linux distribution and print system information next to the ASCII art.
```bash
./target/release/kfetch
```

## Install

Installing kfetch is straightforward simply run
```bash
make install
```

## Configuration

Right now, you can only set to modify the ASCII Art of kfetch, do so by running these commands:
```bash
mkdir -p .config/kfetch/ && vim .config/kfetch/kfetch.conf
```

To modify write:
```bash
distro=Arch (or whatever else is supported)
```

Example:
<img width="587" height="343" alt="Image" src="https://github.com/user-attachments/assets/fe0a7f60-6713-4f0f-9aa2-d22a9b8cb34e" />
