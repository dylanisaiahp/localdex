<div align="center">

# 🔍 ldx — localdex

**A blazing-fast file search CLI for Windows, Linux, and macOS**

[![Version](https://img.shields.io/badge/version-0.0.2-blue.svg)](https://github.com/dylanisaiahp/localdex)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/dylanisaiahp/localdex)

*Find any file on your system in seconds. Search by name, extension, or count everything — with real-time stats.*

</div>

---

## ✨ Features

- **Blazing fast** — parallel multi-threaded directory traversal using the `ignore` crate
- **Flexible search** — substring pattern matching (AhoCorasick), exact extension filtering, or count all files
- **Cross-platform** — Windows, Linux, and macOS with OS-specific flags handled automatically
- **Smart output** — color-coded results, comma-formatted numbers, optional stats
- **Config-driven** — all flags defined in `config.toml` next to the binary, auto-generated on first run
- **Pipeline-friendly** — exit code `0` on match, `1` on no match (like `grep`)
- **Launch files** — open or launch any found file directly from the terminal
- **Multi-drive scanning** — scan all drives at once with per-drive breakdown *(Windows)*

---

## 📦 Installation

### From source

```bash
git clone https://github.com/dylanisaiahp/localdex
cd localdex
cargo build --release
```

The binary will be at `target/release/localdex` (or `localdex.exe` on Windows).

### Windows quick setup

Copy `localdex.exe` to a folder in your PATH (e.g. `C:\Tools\`) and optionally alias it:

```powershell
copy C:\Tools\localdex.exe C:\Tools\ldx.exe
```

On first run, `config.toml` will be auto-generated next to the binary.

---

## 🚀 Usage

```
ldx [pattern] [options]
```

### Examples

```powershell
# Find files containing "invoice" in the name
ldx invoice

# Find all .pdf files quietly (just the count)
ldx -e pdf -q

# Find .rs files in a specific directory
ldx -e rs -d D:\Development

# Count every file on C:\
ldx -a -d C:

# Count every file on all drives with stats
ldx -A -a -S

# Find and launch a game executable
ldx vintagestory.exe --open -1

# Stop after the first 5 matches
ldx -e log -L 5

# Use 8 threads for optimal performance
ldx -a -S -t 8
```

---

## 🏁 Flags

| Flag | Long | Description | OS |
|------|------|-------------|-----|
| `-a` | `--all-files` | Count all files, no filter needed | All |
| `-A` | `--all-drives` | Scan all drives with per-drive breakdown and total | Windows |
| `-d` | `--dir` | Directory to search (default: current) | All |
| `-e` | `--extension` | Search by file extension (e.g. `pdf`, `rs`) | All |
| `-1` | `--first` | Stop after the first match | All |
| `-h` | `--help` | Show help message | All |
| `-L` | `--limit` | Stop after N matches (e.g. `-L 5`) | All |
| `-o` | `--open` | Open or launch the matched file | All |
| `-q` | `--quiet` | Suppress per-file output; still prints summary | All |
| `-s` | `--case-sensitive` | Case-sensitive search | All |
| `-S` | `--stats` | Show scan statistics | All |
| `-t` | `--threads` | Number of threads to use (default: all available) | All |
| `-v` | `--verbose` | Show detailed scan breakdown (files + dirs separately) | All |

> **Tip:** `-e pdf` matches only `.pdf` files. `"pdf"` as a pattern matches anything with `pdf` in the filename (e.g. `Windows.Data.Pdf.dll`). Use `-e` when you want actual files of that type.

---

## ⚙️ Configuration

On first run, `config.toml` is automatically created next to the binary. This file defines all flags and their descriptions — the help text is generated directly from it.

```toml
# ldx configuration file
# Edit this file to customise flags and behaviour
#
# os values: "all", "windows", "linux", "macos"

[flags.all-files]
short = "a"
long = "all-files"
description = "Count all files, no filter needed"
os = "all"
```

You can edit descriptions, and future versions will support user-defined aliases and scripting.

---

## 📊 Benchmarks

All benchmarks performed on:

| Component | Spec |
|-----------|------|
| **Motherboard** | MSI MS-7D37 |
| **CPU** | Intel Core i5-13400F (10 physical / 16 logical cores) |
| **RAM** | 32GB |
| **GPU** | NVIDIA GeForce RTX 3060 8GB |
| **OS** | Windows 11 64-bit |
| **C:** | 500GB SSD (OS drive) |
| **D:** | 1TB SSD (data drive) |
| **G:** | 2TB Seagate BUP Slim BK External HDD (USB) |

---

### 🌡️ Cold vs Warm Cache (SSD)

> Cold = first run after reboot. Warm = OS has cached directory metadata in RAM.

| Drive | Type | Entries | Cold | Warm |
|-------|------|---------|------|------|
| C: (500GB) | SSD | 944,623 | 162,696/s | ~470,000/s |
| D: (1TB) | SSD | 37,042 | 213,995/s | ~700,000/s |
| G: (2TB) | HDD (USB) | 25,760 | 4,280/s | 1,212,282/s |

> The HDD cold vs warm gap is dramatic — **283x slower cold than warm**. Windows aggressively caches HDD directory metadata; once warm it outperforms SSD cold numbers.

> ⚠️ *HDD benchmark was performed on a nearly empty drive (~20k files). Real-world performance on a full, fragmented HDD will vary significantly and is likely slower. HDD benchmarks with a full drive coming in a future update — contributions welcome!*

---

### 🧵 Thread Scaling (C: SSD, warm cache)

| Threads | Time | Speed |
|---------|------|-------|
| 1 | 3.746s | 138,474/s |
| 2 | 2.189s | 236,938/s |
| 4 | 1.307s | 396,762/s |
| **8** | **0.965s** | **537,339/s** ✅ sweet spot |
| 12 | 1.071s | 484,324/s |
| 16 | 2.540s | 204,222/s |

> **8 threads is the sweet spot** on this i5-13400F. Beyond 8, threads compete for IO bandwidth and performance degrades. Use `-t 8` for optimal performance on similar hardware — your mileage may vary, benchmark with `-t` to find your system's sweet spot!

---

### 🏎️ Highlight Results

| Scan | Result | Time | Speed |
|------|--------|------|-------|
| All files on C: (warm, 8t) | 755,642 files | 0.965s | 537,339/s |
| All files on D: (warm) | 31,245 files | 0.052s | 744,363/s |
| All files, Program Files (warm) | 86,080 files | 0.087s | **1,093,053/s** 🏆 |
| All drives combined (warm, 8t) | 786,887 files | 2.477s | 396,149/s |
| Full C: cold scan | 755,816 files | 5.806s | 162,696/s |
| External HDD cold | 20,489 files | 6.018s | 4,280/s |

---

### 🐧 Linux / macOS Benchmarks

*Coming soon — contributions welcome! If you run ldx on Linux or macOS, please open an issue with your benchmark results.*

---

## 🔧 Build Script (Windows)

A `build.ps1` script is included for quick build and deploy:

```powershell
.\build.ps1
```

This builds in release mode, copies the binary to `C:\Tools\`, creates the `ldx.exe` alias, and copies `config.toml`.

---

## 🗺️ Roadmap

### v0.0.3
- Split into multiple source files
- Config-based scripting and user-defined aliases
- `-g/--goto` — navigate to matched file's directory
- Comma number formatting improvements

### Future
- `GSX` — companion tool for game discovery (Steam, Epic, standalone)
- Install script with automatic PATH setup
- Linux/macOS benchmarks

---

## ⚠️ Disclaimer

This software is provided "as is", without warranty of any kind, express or implied. The author is not responsible for any damages, data loss, or consequences arising from the use of this software. This includes but is not limited to:

- Launching, opening, or executing files found by this tool
- Scanning, reading, or indexing files on your system
- Any use of the `--open` flag to launch applications or files
- Actions taken based on search results

**You are solely responsible for how you use this tool.** Always verify files before opening or executing them. This software is licensed under the [MIT License](LICENSE) — use at your own risk.

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

<div align="center">

Built with ❤️ and Rust by [dylanisaiahp](https://github.com/dylanisaiahp)

*If ldx helped you, consider giving it a ⭐ on GitHub!*

</div>
