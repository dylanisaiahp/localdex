<div align="center">

# 🔍 ldx — localdex

**Blazing-fast parallel file search for Windows, Linux, and macOS**

[![Version](https://img.shields.io/badge/version-0.0.8-blue.svg)](https://github.com/dylanisaiahp/localdex)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/dylanisaiahp/localdex)

*Find any file on your system in milliseconds — config-driven, alias-powered, cross-platform.*

**Peak: 7,065,858 entries/s on Linux · 1,641,700 entries/s on Windows**

</div>

---

> **Note:** ldx is the CLI front-end for the upcoming `parex` parallel search engine. After engine separation, ldx will be renamed to `prx`. See [Roadmap](#roadmap).

---

## 📦 Installation

```bash
curl -sSf https://raw.githubusercontent.com/dylanisaiahp/localdex/main/scripts/install.sh | bash
```

> **Windows:** Run via [Git Bash](https://gitforwindows.org/).

Or build from source:

```bash
git clone https://github.com/dylanisaiahp/localdex
cd localdex
./scripts/build.sh
```

---

## 🚀 Quick Start

```bash
ldx invoice                        # find files with "invoice" in the name
ldx -e rs -d ~/projects            # find all .rs files in a directory
ldx -e pdf -q                      # count all PDFs quietly
ldx vintagestory -o -1             # find and launch a file instantly
ldx localdex -D -w                 # find a directory, print cd hint
ldx -a -S -d C:\                   # count every file on C:\ with stats
ldx -e log -L 5                    # stop after 5 matches
ldx main.rs --exclude target       # search, skipping the target/ dir
```

---

## 🐝 Common Flags

| Flag | Long | Description |
|------|------|-------------|
| `-e` | `--extension` | Search by file extension (e.g. `pdf`, `rs`) |
| `-d` | `--dir` | Directory to search (default: current) |
| `-D` | `--dirs` | Search for directories instead of files |
| `-1` | `--first` | Stop after first match |
| `-L` | `--limit` | Stop after N matches |
| `-o` | `--open` | Open the matched file |
| `-w` | `--where` | Print path with cd hint |
| `-q` | `--quiet` | Suppress per-file output |
| `-S` | `--stats` | Show scan statistics |
| `-a` | `--all-files` | Count all files, no filter |

<details>
<summary>Advanced flags</summary>

| Flag | Long | Description | OS |
|------|------|-------------|-----|
| `-A` | `--all-drives` | Scan all drives with per-drive breakdown | Windows |
| `-s` | `--case-sensitive` | Case-sensitive search | All |
| `-t` | `--threads` | Thread count (default: all logical cores) | All |
| `-v` | `--verbose` | Files + dirs breakdown in stats | All |
| `-h` | `--help` | Dynamic help — shows your aliases too | All |
| | `--exclude` | Skip directories (comma-separated) | All |
| | `--check` | Validate config | All |
| | `--sync` | Add missing default flags to config | All |
| | `--reset` | Restore default flags (keeps aliases) | All |
| | `--edit` | Open config in editor | All |
| | `--config` | Print config location | All |
| | `--version` | Show version | All |

</details>

> `-d` sets *where* to search. `-D` searches *for* directories. `-s` = case-sensitive, `-S` = stats.

---

## ⚙️ Configuration

`config.toml` is auto-generated on first install. Every flag is remappable. Add aliases and custom flags to make ldx your own.

```toml
[aliases]
repo = "localdex -D -d D: -1 -S -w -q"
pdf  = "-e pdf -q"

[custom.rs]
short = "R"
long = "rust"
description = "Search for Rust files"
os = "all"
action = "set_value"
target = "extension"
value = "rs"
```

```bash
ldx --check   # validate config
ldx --sync    # restore any missing default flags
ldx --reset   # reset flags to defaults (keeps aliases & custom)
```

---

## 📊 Benchmarks

| Platform | Hardware | Peak Speed |
|----------|----------|------------|
| Linux (CachyOS) | Ryzen 7 5825U, NVMe | **7,065,858 entries/s** |
| Windows 11 | i5-13400F, SSD | **1,641,700 entries/s** |

See [BENCHMARKS.md](BENCHMARKS.md) for full thread scaling tables and Linux vs Windows comparisons.

---

## 📄 Docs

| | |
|--|--|
| [ROADMAP.md](ROADMAP.md) | What's next |
| [CHANGELOG.md](CHANGELOG.md) | Version history |
| [BENCHMARKS.md](BENCHMARKS.md) | Performance data |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |


---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Linux and macOS benchmark results especially welcome!

---

## ⚠️ Disclaimer & License

MIT — see [LICENSE](LICENSE). Provided as-is; use responsibly.

---

<div align="center">

Built with ❤️ and Rust by [dylanisaiahp](https://github.com/dylanisaiahp)

*If ldx helped you, consider giving it a ⭐!*

</div>
