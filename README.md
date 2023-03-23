<h1 align="center">pfetch-rs</h1>
<p align="center">A rewrite of the pfetch system information tool by dylanaraps in Rust</p><br>
<p align="center"><img src="https://user-images.githubusercontent.com/50576978/219375863-579c495d-8db8-4aa9-a4a6-348ecb2c849f.png" width="350px"></p>

## About

If you are familiar with the [pfetch](https://github.com/dylanaraps/pfetch)
system information tool by [dylanaraps](https://github.com/dylanaraps), this
does the exact same thing, but with an about _10x faster_ runtime. _pfetch_ is
simple by design with some (but not many) configuration options and a
minimalistic look.

**Supported Platforms:** Linux, Android, macOS, Windows, FreeBSD, NetBSD

**Included Logos:** Alpine Linux, Android, AmogOS _(new)_ Arch Linux, ArcoLinux,
Artix Linux, Bedrock Linux, Buildroot, CelOS, CentOS, Crystal Linux, dahliaOS,
Debian, Devuan, DietPi _(new)_, DragonflyBSD, Elementary OS, EndeavourOS,
Fedora, Fiwix _(new)_, FreeBSD, Garuda Linux, Gentoo Linux, Gnu, Guix, Haiku,
HydroOS, Hyperbola, instantOS, IRIX, KDE neon, Linux Lite, Linux, Mint, macOS,
Mageia, Manjaro, Minix, MorphOS _(new)_, MX Linux, NetBSD, NixOS, Nobara Project
_(new)_, OpenBSD, openSUSE Tumbleweed, openSUSE Leap, OpenWrt, Parabola,
Pop!\_OS _(updated)_, PureOS, Raspbian, SerenityOS, Slackware, Solus, SteamOS
_(new)_, Solaris, Ubuntu, Vanilla OS _(new)_, Void Linux, Windows _(new)_,
Xeonix Linux

For all other distributions, a penguin will be displayed.

_Credit to [the original pfetch](https://github.com/dylanaraps/pfetch) and
[its contributors](https://github.com/dylanaraps/pfetch/graphs/contributors)._

If you want to add a logo, feel free to make a Pull Request.

## Status

This project is still in early development, expect things to not work properly.
Please open issues for bugs you are encountering.

## Installation

### Binary

Download a binary from the
[latest release](https://github.com/Gobidev/pfetch-rs/releases/latest).

### Cargo

```sh
cargo install pfetch
```

### Homebrew

```sh
brew install pfetch-rs
```

### AUR

Install the [pfetch-rs](https://aur.archlinux.org/packages/pfetch-rs) or
[pfetch-rs-bin](https://aur.archlinux.org/packages/pfetch-rs-bin) AUR package.

## Performance

Benchmarks performed on an AMD Ryzen 5 3600. Execution time is measured using
[hyperfine](https://github.com/sharkdp/hyperfine) with `-w 4 -m 500 -N` flags

|  Implementation   | Mean [ms]  | Min [ms] | Max [ms] |
| :---------------: | :--------: | :------: | :------: |
| POSIX `sh` (bash) | 23.7 ± 0.9 |   22.3   |   29.3   |
| POSIX `sh` (dash) | 15.9 ± 0.3 |   15.1   |   18.2   |
|   Rust (v2.3.0)   | 2.2 ± 0.2  |   1.8    |   3.9    |

_Note: This is with `pacman` and `flatpak` being the only installed package
managers. Especially having `nix` installed will have a big impact on
performance, as querying installed `nix` packages is very costly. If you want to
skip slow package managers from being counted, you can set the
`PF_FAST_PKG_COUNT` environment variable._

## Configuration

Like the original `pfetch`, `pfetch-rs` is configured through environment
variables. Your existing config will probably still work, the only difference is
how padding is configured.

If you want to display a custom logo, you will have to download the source code,
make your changes to `./pfetch-extractor/logos.sh` and build the binary with
`cargo b --release`.

```sh
# Which information to display.
# Default: first example below
# Valid: space separated string
#
# OFF by default: shell editor wm de palette
PF_INFO="ascii title os host kernel uptime pkgs memory"

# Example: Only ASCII.
PF_INFO="ascii"

# Example: Only Information.
PF_INFO="title os host kernel uptime pkgs memory"

# A file containing environment variables to source before running pfetch.
# Default: unset
# Valid: A shell script
PF_SOURCE=""

# Separator between info name and info data.
# Default: unset
# Valid: string
PF_SEP=":"

# Enable/Disable colors in output:
# Default: 1
# Valid: 1 (enabled), 0 (disabled)
PF_COLOR=1

# Color of info names:
# Default: unset (auto)
# Valid: 0-9
PF_COL1=4

# Color of info data:
# Default: unset (auto)
# Valid: 0-9
PF_COL2=9

# Color of title data:
# Default: unset (auto)
# Valid: 0-9, COL1 (copies COL1 value)
PF_COL3=1

# Alignment paddings (this is different to the original version).
# Default: unset (auto)
# Valid: int
PF_PAD1=""
PF_PAD2=""
PF_PAD3=""

# Which ascii art to use.
# Default: unset (auto)
# Valid: string
PF_ASCII="openbsd"

# The below environment variables control more
# than just 'pfetch' and can be passed using
# 'HOSTNAME=cool_pc pfetch' to restrict their
# usage solely to 'pfetch'.

# Which user to display.
USER=""

# Which hostname to display.
HOSTNAME=""

# Skip package managers that take "long" to query package count (like nix)
PF_FAST_PKG_COUNT=1
```
