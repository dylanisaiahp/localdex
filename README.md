<div align="center">

# 🔍 ldx — localdex

**A blazing-fast file search CLI for Windows, Linux, and macOS**

[![Version](https://img.shields.io/badge/version-0.0.7-blue.svg)](https://github.com/dylanisaiahp/localdex)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/dylanisaiahp/localdex)

*Find any file on your system in seconds. Search by name, extension, or directory — with real-time stats and full config control.*

</div>

---

## ✨ Features

- **Blazing fast** — parallel multi-threaded directory traversal using the `ignore` crate
- **Flexible search** — files, directories, extensions, or count everything
- **Cross-platform** — Windows, Linux, and macOS with OS-specific flags handled automatically
- **Smart output** — color-coded results, comma-formatted numbers, optional stats
- **Config-driven** — all flags defined in `config.toml`, auto-generated on first run
- **Pipeline-friendly** — exit code `0` on match, `1` on no match (like `grep`)
- **Launch files** — open or launch any found file directly from the terminal
- **Multi-drive scanning** — scan all drives at once with per-drive breakdown *(Windows)*
- **Aliases** — define shorthand commands that expand to full flag sets
- **Custom flags** — map your own short flags to built-in behaviors
- **Config management** — `--check`, `--sync`, `--reset` keep your config healthy
- **Dynamic help** — `--help` shows your aliases and custom flags alongside built-ins

---

## 📦 Installation

```bash
curl -sSf https://raw.githubusercontent.com/dylanisaiahp/localdex/main/install.sh | bash
```

> **Windows users:** Run via [Git Bash](https://gitforwindows.org/).

Or manually:

```bash
git clone https://github.com/dylanisaiahp/localdex
cd localdex
./build.sh
```

---

## 🚀 Usage

```
ldx [pattern] [options]
```

```bash
ldx invoice                  # find files with "invoice" in the name
ldx -e pdf -q                # count all .pdf files quietly
ldx -e rs -d D:\Development  # find .rs files in a specific directory
ldx -a -S -d C:\             # count every file on C:\ with stats
ldx -A -a -S                 # count every file on all drives
ldx vintagestory -o -1       # find and launch a file
ldx localdex -D -w           # find a directory and print cd command
ldx -e log -L 5              # stop after 5 matches
ldx main.rs --exclude target # search excluding target directory
ldx --config                 # show config location
ldx --edit                   # open config in editor
ldx --check                  # validate config and print summary
ldx --sync                   # add any missing default flags
ldx --reset                  # restore default flags (preserves aliases)
```

---

## 🐝 Flags

| Flag | Long | Description | OS |
|------|------|-------------|-----|
| `-a` | `--all-files` | Count all files, no filter needed | All |
| `-A` | `--all-drives` | Scan all drives with per-drive breakdown and total | Windows |
| `-d` | `--dir` | Directory to search in (default: current) | All |
| `-D` | `--dirs` | Search for directories instead of files | All |
| `-e` | `--extension` | Search by file extension (e.g. `pdf`, `rs`) | All |
| `-1` | `--first` | Stop after the first match | All |
| `-h` | `--help` | Show help message (dynamic — shows your aliases) | All |
| `-L` | `--limit` | Stop after N matches (e.g. `-L 5`) | All |
| `-o` | `--open` | Open or launch the matched file | All |
| `-q` | `--quiet` | Suppress per-file output; still prints summary | All |
| `-s` | `--case-sensitive` | Case-sensitive search | All |
| `-S` | `--stats` | Show scan statistics | All |
| `-t` | `--threads` | Number of threads (default: all available, capped at logical core count) | All |
| `-v` | `--verbose` | Show detailed scan breakdown (files + dirs separately) | All |
| `-w` | `--where` | Print path with cd hint (implies `-1`) | All |
| | `--check` | Validate config — reports duplicate flags, missing targets, bad custom flags | All |
| | `--config` | Print config file location | All |
| | `--edit` | Open config in default editor | All |
| | `--exclude` | Skip directories (e.g. `--exclude node_modules,target,.git`) | All |
| | `--reset` | Restore default flags, preserving your aliases and custom flags | All |
| | `--sync` | Add any missing default flags to your config without overwriting customizations | All |
| | `--version` | Show version | All |

> **Note:** `-d` sets *where* to search. `-D` searches *for* directories. Similarly, `-s` is case-sensitive and `-S` shows stats.

> **Tip:** `-e pdf` matches only `.pdf` files. `"pdf"` as a pattern matches anything with `pdf` in the filename.

---

## ⚙️ Configuration

On first run, `config.toml` is auto-generated next to the binary. All flags and their descriptions are defined here — help text is generated directly from it. Every flag's short/long name is remappable.

```toml
[flags.stats]
short = "S"
long = "stats"
description = "Show scan statistics"
os = "all"
action = "set_boolean"
target = "stats"

# User-defined aliases
[aliases]
repo = "localdex -D -d D: -1 -S -w -q"
ct   = "-a -A -S -q --verbose"

# Custom flags
[custom.pdf]
short = "P"
long = "pdf"
description = "Search for PDF files"
os = "all"
action = "set_value"
target = "extension"
value = "pdf"
```

### Config management commands

```bash
ldx --check   # validate config: catch duplicate flags, bad targets, missing fields
ldx --sync    # add any flags present in defaults but missing from your config
ldx --reset   # restore all [flags] to defaults — aliases and custom flags are kept
```

---

## 📊 Benchmarks

Peak result: **1,641,700 entries/s** on Windows SSD (i5-13400F, 16 threads).

See [BENCHMARKS.md](BENCHMARKS.md) for full thread scaling tables, cold vs warm cache data, and hardware specs.

---

## 🔧 Scripts

| Script | Description |
|--------|-------------|
| `install.sh` | Install or update ldx — detects existing installs, checks for updates |
| `build.sh` | Build and deploy ldx |
| `benchmark.sh` | Benchmark across directories and thread counts — outputs CSV |
| `uninstall.sh` | Cleanly remove ldx from your system |
| `scripts/bump.sh` | Suggest and apply version bumps based on git log and roadmap |

All scripts run on Windows via [Git Bash](https://gitforwindows.org/).

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for build instructions, benchmark guidelines, and PR requirements.

Linux and macOS benchmark results are especially valuable — open an issue or PR with your CSV!

---

## 🗺️ Roadmap

### v0.0.7 ✓ Shipped
- `--check`, `--sync`, `--reset` config management
- Dynamic `--help` showing user's aliases and custom flags
- Nicer help layout with Management section and Tips
- `-L` limit race condition fixed
- Version sourced from `Cargo.toml`

### v0.0.8 (next)
- README and docs updated to v0.0.7
- Full flag test pass (Windows + Linux)
- `bump.sh` version helper
- Clean release commit + tag

### v0.1.0 (beta)
- Codebase audit — cut bloat, improve clarity
- Unit tests for edge cases
- Full Linux and macOS benchmark data

### Future
- Pre-built binaries for Windows, Linux, and macOS
- GUI — Spotlight-style, streaming results, plugin themes
- Core engine separation (`parex`) for third-party integration
- GitHub Actions CI

---

## 📢 Version Scheme

| Range | Stage | Description |
|-------|-------|-------------|
| `v0.0.X` | **experimental** | Early development, rapid iteration |
| `v0.X.X` | **beta** | Code polish, engine separation, GUI design |
| `r1.0` | **stable release** | Production-ready, battle-tested |
| `r1.X` | **refinements** | Minor updates and fixes |
| `r2.0` | **major leap** | Significant performance or feature breakthroughs |

> The `r` prefix signals "production-ready." Clean, simple versioning.

---

## ⚠️ Disclaimer

This software is provided "as is" without warranty. The author is not responsible for any damages arising from use of this tool. **You are solely responsible for how you use ldx.** Licensed under [MIT](LICENSE).

---

## 📄 License

MIT — see [LICENSE](LICENSE) for details.

---

<div align="center">

Built with ❤️ and Rust by [dylanisaiahp](https://github.com/dylanisaiahp)

*If ldx helped you, consider giving it a ⭐ on GitHub!*

</div>
