<h1 align="center">pfetch-rs</h1>
<p align="center">A rewrite of the pfetch system information tool by dylanaraps in Rust</p><br>
<p align="center"><img src="https://user-images.githubusercontent.com/50576978/219375863-579c495d-8db8-4aa9-a4a6-348ecb2c849f.png" width="350px"></p>

## About

If you are familiar with the [pfetch](https://github.com/dylanaraps/pfetch)
system information tool by [dylanaraps](https://github.com/dylanaraps), this
does the exact same thing, but with an about _10x faster_ runtime. _pfetch_ is
simple by design with some (but not many) configuration options and a
minimalistic look.

**Supported Platforms:** Linux, Android, DragonflyBSD, FreeBSD, NetBSD, OpenBSD,
WSL, Haiku, MacOS, Minix, Solaris, IRIX, SerenityOS

_Disclaimer: Aside from Linux, all of these platforms are untested. If you run
into problems, please open an issue._

**Included Logos:** Alpine Linux, Android, Arch Linux, ArcoLinux, Artix Linux,
Bedrock Linux, Buildroot, CelOS, CentOS, Crystal Linux, dahliaOS, Debian,
Devuan, DragonflyBSD, Elementary OS, EndeavourOS, Fedora, FreeBSD, Garuda Linux,
Gentoo Linux, Gnu, Guix, Haiku, HydroOS, Hyperbola, instantOS, IRIX, KDE neon,
Linux Lite, Linux, Mint, macOS, Mageia, Manjaro, Minix, MX Linux, NetBSD, NixOS,
OpenBSD, openSUSE Tumbleweed, openSUSE Leap, OpenWrt, Parabola, Pop!\_OS
(updated), PureOS, Raspbian, SerenityOS, Slackware, Solus, Solaris, Ubuntu, Void
Linux, Xeonix Linux, Fiwix (new), MorphOS (new), AmogOS (new), Aperio (new)

For all other distributions, a penguin will be displayed.

_Credit to [the original pfetch](https://github.com/dylanaraps/pfetch) and
[its contributors](https://github.com/dylanaraps/pfetch/graphs/contributors)._

If you want to add a logo, feel free to make a Pull Request.

## Status

This project is still in early development, expect things to not work properly.
Please open issues for bugs you are encountering.

## Installation

### Cargo

```sh
cargo install pfetch
```

### AUR

Install the [pfetch-rs](https://aur.archlinux.org/packages/pfetch-rs) AUR
package

## Performance

Benchmarks performed on an AMD Ryzen 5 3600. Execution time is measured using
[hyperfine](https://github.com/sharkdp/hyperfine) with `-w 4 -m 500 -N` flags

|  Implementation   | Mean [ms]  | Min [ms] | Max [ms] |
| :---------------: | :--------: | :------: | :------: |
| POSIX `sh` (bash) | 27.3 ± 0.9 |   25.3   |   23.2   |
| POSIX `sh` (dash) | 19.3 ± 0.6 |   18.3   |   24.0   |
|       Rust        | 2.1 ± 0.2  |   1.8    |   3.6    |

_Note: This is with `pacman` being the only installed package manager.
Especially having `nix` installed will have a big impact on performance, as
querying installed `nix` packages is very costly._

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
# Valid: 0-9
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

# Which editor to display.
EDITOR=""

# Which shell to display.
SHELL=""

# Which desktop environment to display.
XDG_CURRENT_DESKTOP=""

```
