<div align="center">

# 🔍 ldx — localdex

**Blazing-fast parallel file search for Windows, Linux, and macOS**

[![Version](https://img.shields.io/badge/version-0.3.4-blue.svg)](https://github.com/dylanisaiahp/localdex)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/dylanisaiahp/localdex)

*Find any file on your system in milliseconds — config-driven, alias-powered, cross-platform.*

**Linux: ~4.3M entries/s on / (Ryzen 9 7950X, t=16)**

</div>

---

## 📦 Installation

```bash
curl -sSf https://raw.githubusercontent.com/dylanisaiahp/localdex/main/install.sh | sh
```

> **Windows:** Run via [Git Bash](https://gitforwindows.org/).

---

## 🚀 Quick Start

```bash
ldx invoice                        # find files matching "invoice"
ldx -e rs -d ~/projects            # find all .rs files in a directory
ldx -e pdf -q                      # count all PDFs quietly
ldx vintagestory -o -1             # find and open a file instantly
ldx localdex -D -w                 # find a directory, print cd hint
ldx -a -S -d /                     # count every file with stats
ldx -e log -L 5                    # stop after 5 matches
ldx main.rs --exclude target       # skip the target/ directory
```

---

## 🐝 Flags

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
| `-s` | `--case-sensitive` | Case-sensitive search |
| `-t` | `--threads` | Thread count (default: all logical cores) |
| `-v` | `--verbose` | Files + dirs breakdown in stats |
| `-a` | `--all-files` | Count all files, no filter |
| `-A` | `--all-drives` | Scan all drives (Windows) |
|      | `--exclude` | Skip directories (comma-separated) |
| `-W` | `--warn` | Show skipped paths |

**Management:**

```bash
ldx --check     # validate config
ldx --sync      # restore any missing default flags
ldx --reset     # reset flags to defaults (keeps aliases & custom)
ldx --edit      # open config in editor
ldx --config    # print config file location
```

**Benchmarking:**

```bash
ldx bench                          # benchmark against $HOME, /usr, /
ldx bench --runs 10 --live         # live output, 10 runs per dir
ldx bench --dirs ~/projects        # add extra directories
ldx bench --csv                    # also save a CSV
```

> `-d` sets *where* to search. `-D` searches *for* directories. `-s` = case-sensitive, `-S` = stats.

---

## ⚙️ Configuration

`config.toml` is generated automatically on install. Every flag is remappable. Add aliases and custom flags to make ldx yours.

```toml
[aliases]
repo = "localdex -D -d ~ -1 -S -w -q"

[custom.rust]
short = "R"
long = "rust"
description = "Search for Rust files"
os = "all"
action = "set_value"
target = "extension"
value = "rs"
```

---

## 🏗️ Architecture

ldx is built on **[parex](https://crates.io/crates/parex)** — a dedicated parallel search engine library published separately to crates.io.

```
ldx (CLI)
 ├── flags.rs      — argument parsing
 ├── config.rs     — config loading
 ├── source.rs     — DirectorySource (implements parex::Source)
 ├── search.rs     — thin wrapper around parex::search()
 ├── display.rs    — output formatting
 ├── launcher.rs   — OS file opener
 └── bench*.rs     — built-in benchmarker

parex (engine)
 ├── Source trait  — walk anything: files, databases, memory
 ├── Matcher trait — substring, extension, fuzzy, custom
 └── SearchBuilder — fluent API, thread control, error collection
```

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Bug reports, benchmark results from different hardware, and platform testing are especially welcome.

---

## ⚠️ License

MIT — see [LICENSE](LICENSE).

---

<div align="center">

Built with ❤️ and Rust by [dylanisaiahp](https://github.com/dylanisaiahp)

*If ldx helped you, consider giving it a ⭐*

</div>
