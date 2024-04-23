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

**Included Logos:** Alma Linux _(new)_, Alpine Linux, Android, AmogOS _(new)_,
Arch Linux, ArcoLinux, Artix Linux, Bazzite _(new)_, Bedrock Linux, Buildroot,
CelOS, CentOS, Crystal Linux, dahliaOS, Debian, Devuan, DietPi _(new)_,
DragonflyBSD, Elementary OS, EndeavourOS, Fedora, Fiwix _(new)_, FreeBSD, Garuda
Linux, Gentoo Linux, Gnu Hurd _(updated)_, Guix, Haiku, HydroOS, Hyperbola,
instantOS, IRIX, KDE neon, Linux Lite, Linux, Mint, macOS, Mageia, Manjaro,
Minix, MorphOS _(new)_, MX Linux, NetBSD, NixOS, Nobara Project _(new)_,
OpenBSD, openSUSE Tumbleweed, openSUSE Leap, OpenWrt, Oracle Linux _(new)_,
Parabola, Pop!\_OS _(updated)_, PureOS, Raspbian, Rocky Linux _(new)_,
SerenityOS, Slackware, Solus, SteamOS _(new)_, Solaris, Ubuntu, Vanilla OS
_(new)_, Void Linux, Windows _(new)_, Xeonix Linux

You can check out how they look [here](./all_logos.md).

For all other distributions, a penguin will be displayed.

_Credit to [the original pfetch](https://github.com/dylanaraps/pfetch) and
[its contributors](https://github.com/dylanaraps/pfetch/graphs/contributors)._

If you want a logo to be added, feel free to open an issue or a PR.

## Installation

_Note: On openSUSE, install the `rpm-devel` package for faster package count._

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

### Nixpkgs

Install the
[pfetch-rs](https://search.nixos.org/packages?channel=unstable&show=pfetch-rs)
Nix package.

### AUR

Install the [pfetch-rs](https://aur.archlinux.org/packages/pfetch-rs) or
[pfetch-rs-bin](https://aur.archlinux.org/packages/pfetch-rs-bin) AUR package.

## Performance

Benchmarks performed on an AMD Ryzen 5 3600. Execution time is measured using
[hyperfine](https://github.com/sharkdp/hyperfine) with `-w 4 -m 500 -N` flags.

|  Implementation   | Mean [ms]  | Min [ms] | Max [ms] |
| :---------------: | :--------: | :------: | :------: |
| POSIX `sh` (bash) | 23.7 ± 0.9 |   22.3   |   29.3   |
| POSIX `sh` (dash) | 15.9 ± 0.3 |   15.1   |   18.2   |
|   Rust (v2.3.0)   | 2.2 ± 0.2  |   1.8    |   3.9    |

_Note: This is with `pacman` and `flatpak` being the only installed package
managers. For more info, see [Improving Performance](#imp_perf)._

<a name="imp_perf"></a>

### Improving Performance

The by far slowest part of the `pfetch` execution time is counting the installed
packages. For most package managers this is still very fast, but there are some
(currently `nix` (and `zypper`, if `rpm-devel` is not installed)) that take
~500ms to report installed packages, which takes away all performance benefits
of the Rust version. If you have one or more of these installed, you can skip
counting them by setting the `PF_FAST_PKG_COUNT` environment variable to any
value.

## Configuration

Like the original `pfetch`, `pfetch-rs` is configured through environment
variables. Your existing config will probably still work, the main difference is
how padding is configured.

If you want to display a custom logo, use the `PF_CUSTOM_LOGOS` option, an
example for a custom logos file can be found below.

```sh
# Which information to display.
# Default: first example below
# Valid: space separated string
#
# OFF by default: shell editor wm de palette cpu
PF_INFO="ascii title os host kernel uptime pkgs memory"

# Example: Only ASCII.
PF_INFO="ascii"

# Example: Only Information.
PF_INFO="title os host kernel uptime pkgs memory"

# A file containing environment variables to source before running pfetch
# Default: unset
# Valid: A shell script
PF_SOURCE=""

# A file containing pfetch logos to overwrite default logos or add new logos
# Default: unset
# Valid: Path to a file containing pfetch logos (example below)
PF_CUSTOM_LOGOS="~/.config/pfetch_logos"

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

A file containing custom pfetch logos could look like this (also found under
`custom_logos_example`). This will turn the Arch Linux logo red, the Debian Logo
blue and the Fedora logo yellow:

```
[Aa]rch*)
	read_ascii 1 <<- EOF
			${c1}       /\\
			${c1}      /  \\
			${c1}     /\\   \\
			${c1}    /      \\
			${c1}   /   ,,   \\
			${c1}  /   |  |  -\\
			${c1} /_-''    ''-_\\
		EOF
	;;
[Dd]ebian*)
	read_ascii 4 <<- EOF
			${c4}  _____
			${c4} /  __ \\
			${c4}|  /    |
			${c4}|  \\___-
			${c4}-_
			${c4}  --_
		EOF
	;;
[Ff]edora*)
    read_ascii 3 <<- EOF
			        ${c3},'''''.
			       ${c3}|   ,.  |
			       ${c3}|  |  '_'
			${c3}  ,....|  |..
			${c3}.'  ,_;|   ..'
			${c3}|  |   |  |
			${c3}|  ',_,'  |
			${c3} '.     ,'
			   ${c3}'''''
		EOF
```

_Note: Make sure to use tabs for indentation and separate logos with `;;`, as
seen above. You only need to add the logos you want to overwrite/add, the
default logos will stay available. The included logos can be found at
`./pfetch-extractor/logos.sh`._
