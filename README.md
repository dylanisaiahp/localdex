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
- **Easy install** — one-line installer with automatic PATH setup, update detection, and 4 install locations

---

## 📦 Installation

### One-line install (recommended)

```bash
curl -sSf https://raw.githubusercontent.com/dylanisaiahp/localdex/main/install.sh | bash
```

> **Windows users:** Run via [Git Bash](https://gitforwindows.org/).

The installer will:
- Detect if ldx is already installed and check for updates
- Build from source (pre-built binaries coming in a future release)
- Let you choose your install location
- Set up PATH automatically if needed

### Manual install

```bash
git clone https://github.com/dylanisaiahp/localdex
cd localdex
./build.sh
```

### Install locations

The installer offers 4 options per platform:

| Option | Windows | Linux / macOS |
|--------|---------|---------------|
| 1 (default) | `%USERPROFILE%\.cargo\bin` ✅ already in PATH | `~/.cargo/bin` ✅ already in PATH |
| 2 | `%USERPROFILE%\bin` | `~/.local/bin` |
| 3 | `C:\Program Files\ldx` | `/usr/local/bin` |
| 4 | Custom | Custom |

> **Note:** Examples in this README use `ldx` assuming it's in your PATH. If you chose a custom location, either add it to PATH or use the full path to the binary.

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

> All benchmarks are statistically derived from **30 warm runs** (10 + 20) and **20 cold runs** per configuration. Averages and medians reported.

---

### 🧵 Thread Scaling — `C:\` (945k entries, SSD)

> The largest and most demanding test — full OS drive scan.

| Threads | Warm Avg | Warm Median | Cold Avg | Cold Median |
|---------|----------|-------------|----------|-------------|
| 1 | 78,007/s | 70,511/s | 114,478/s | 125,380/s |
| 2 | 133,680/s | 133,169/s | 216,171/s | 216,116/s |
| 4 | 353,154/s | 354,510/s | 357,055/s | 357,571/s |
| 6 | 466,596/s | 467,436/s | 469,815/s | 468,738/s |
| 8 | 497,543/s | 498,302/s | 502,630/s | 499,327/s |
| **10** | **526,404/s** | **522,122/s** | **518,317/s** | **515,980/s** ✅ |
| 12 | 516,335/s | 516,611/s | 502,819/s | 502,919/s |
| 14 | 465,166/s | 467,256/s | 459,313/s | 460,702/s |
| 16 | 440,848/s | 437,274/s | 431,975/s | 430,744/s |

> **Sweet spot: 10 threads** on full `C:\`. Beyond 10, IO contention hurts performance.

---

### 🧵 Thread Scaling — `C:\Users\dylan` (520k entries, SSD)

| Threads | Warm Avg | Warm Median | Cold Avg | Cold Median |
|---------|----------|-------------|----------|-------------|
| 1 | 148,005/s | 149,392/s | 163,079/s | 164,582/s |
| 2 | 263,567/s | 269,237/s | 273,696/s | 276,742/s |
| 4 | 450,877/s | 453,103/s | 462,633/s | 462,200/s |
| 6 | 603,919/s | 603,681/s | 618,517/s | 616,414/s |
| 8 | 647,473/s | 663,633/s | 693,803/s | 698,297/s |
| **10** | 731,840/s | 733,470/s | **743,145/s** | **739,625/s** ✅ |
| **12** | **747,916/s** | **752,200/s** | 721,559/s | 715,886/s |
| 14 | 726,490/s | 717,343/s | 699,147/s | 707,634/s |
| 16 | 685,486/s | 688,550/s | 647,972/s | 636,272/s |

> **Sweet spot: 10–12 threads** on home directory. Warm favors 12, cold favors 10.

---

### 🧵 Thread Scaling — `C:\Program Files` (86k entries, SSD)

| Threads | Warm Avg | Warm Median | Cold Avg | Cold Median |
|---------|----------|-------------|----------|-------------|
| 1 | 290,563/s | 297,032/s | 306,049/s | 310,806/s |
| 2 | 509,440/s | 509,217/s | 529,256/s | 529,749/s |
| 4 | 841,393/s | 843,476/s | 867,909/s | 869,182/s |
| 6 | 1,125,433/s | 1,127,601/s | 1,148,363/s | 1,144,717/s |
| 8 | 1,240,273/s | 1,244,297/s | 1,295,313/s | 1,295,280/s |
| 10 | 1,336,502/s | 1,341,616/s | 1,372,124/s | 1,364,888/s |
| 12 | 1,444,864/s | 1,444,834/s | 1,461,014/s | 1,457,803/s |
| 14 | 1,553,522/s | 1,567,294/s | 1,573,560/s | 1,583,581/s |
| **16** | **1,618,843/s** | **1,617,653/s** | **1,641,700/s** | **1,646,164/s** ✅ |

> **Sweet spot: 16 threads** on smaller directories. Small datasets don't saturate IO so more threads always win.

---

### 🧵 Thread Scaling — `D:\` (37k entries, SSD)

| Threads | Warm Avg | Warm Median | Cold Avg | Cold Median |
|---------|----------|-------------|----------|-------------|
| 1 | 153,285/s | 155,817/s | 156,267/s | 162,360/s |
| 2 | 263,950/s | 268,110/s | 278,817/s | 279,628/s |
| 4 | 443,506/s | 445,309/s | 453,977/s | 455,020/s |
| 6 | 583,630/s | 592,631/s | 593,988/s | 596,992/s |
| 8 | 641,250/s | 644,592/s | 661,291/s | 662,264/s |
| 10 | 675,445/s | 688,288/s | 707,140/s | 706,158/s |
| 12 | 685,660/s | 701,194/s | 750,338/s | 755,772/s |
| 14 | 704,422/s | 720,035/s | 791,790/s | 795,886/s |
| **16** | **749,385/s** | **744,496/s** | **815,817/s** | **817,494/s** ✅ |

> **Sweet spot: 16 threads** on the smaller data drive. Consistent with the pattern — smaller datasets benefit from more threads.

---

### 💡 Thread Scaling Summary

The sweet spot depends on how many files you're scanning:

| Dataset Size | Recommended Threads |
|---|---|
| < 100k entries | 16 (more = better) |
| 100k – 500k entries | 10–12 |
| 500k+ entries | 10 |

> Use `-t` to benchmark your own system — results vary by CPU, drive speed, and dataset size!

---

### 🌡️ Cold vs Warm Cache

> Cold = first run after reboot. Warm = OS has cached directory metadata in RAM.

| Directory | Entries | Cold (10t) | Warm (10t) | Speedup |
|-----------|---------|------------|------------|---------|
| `C:\` | 945k | 518,317/s | 526,404/s | ~1x |
| `C:\Users\dylan` | 520k | 743,145/s | 731,840/s | ~1x |
| `C:\Program Files` | 86k | 1,372,124/s | 1,336,502/s | ~1x |
| `D:\` | 37k | 707,140/s | 675,445/s | ~1x |

> On SSD, cold and warm cache performance is nearly identical — the OS caches SSD metadata so efficiently that even "cold" scans are fast.

---

### 🏎️ Peak Results

| Scan | Entries | Speed |
|------|---------|-------|
| `C:\Program Files` cold, 16t | 86k | **1,641,700/s** 🏆 |
| `C:\Program Files` warm, 16t | 86k | 1,618,843/s |
| `C:\Users\dylan` cold, 10t | 520k | 743,145/s |
| `C:\` warm, 10t | 945k | 526,404/s |
| `D:\` cold, 16t | 37k | 815,817/s |

---

### 🖴 HDD / External Drive Benchmarks

HDD and external drive performance varies significantly based on drive age, fragmentation, and how full it is — so we haven't included any numbers here. If you have real-world HDD benchmark results, run `benchmark.sh` and open an issue or PR with your CSV!

---

### 🐧 Linux / macOS Benchmarks

*Coming soon — contributions welcome! If you run ldx on Linux or macOS, please open an issue with your benchmark results.*

---

## 🔧 Scripts

| Script | Description |
|--------|-------------|
| `install.sh` | Install or update ldx — detects existing installs, checks for updates, builds from source |
| `build.sh` | Build and deploy ldx — supports `--debug`, `--release`, `--dest` |
| `benchmark.sh` | Benchmark ldx across directories and thread counts — outputs labeled CSV |

All scripts are cross-platform and run on Windows via [Git Bash](https://gitforwindows.org/).

```bash
# Install
./install.sh

# Build with options
./build.sh --release --dest ~/.local/bin

# Benchmark (warm cache, 20 runs)
./benchmark.sh --runs 20 --warm

# Benchmark (cold cache — run immediately after reboot)
./benchmark.sh --runs 20 --cold
```

---

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for build instructions, benchmark guidelines, and PR requirements.

If you run ldx on Linux or macOS, your benchmark results are especially valuable — open an issue or PR with your CSV files!

---

## 🗺️ Roadmap

### v0.0.3
- Split into multiple source files
- Config-based scripting and user-defined aliases
- `-g/--goto` — navigate to matched file's directory

### Future
- `GSX` — companion tool for game discovery (Steam, Epic, standalone)
- Pre-built binaries for Windows, Linux, and macOS
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
