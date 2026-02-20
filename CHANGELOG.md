# Changelog

All notable changes to ldx are listed here. Newest first.

---

## v0.2.0 — 2026-02-21 (Engine Overhaul)
- Integrated `parex v0.1.0` — ldx now runs on a dedicated parallel search engine
- Added `source.rs` — `DirectorySource` implements `parex::Source` for filesystem walks
- Gutted `search.rs` — replaced ~255 lines of walk logic with thin `parex::search()` wrapper
- Split `config.rs` (566 lines) → `config.rs` (~80 lines) + `config_check.rs` (~260 lines)
- Extracted `DEFAULT_CONFIG` to `default_config.toml` — `include_str!` at compile time
- Refactored `flags.rs` — `parse_args` split into `parse_management`, `parse_value_flags`, `parse_bool_flags`, `parse_pattern`, `validate_combos`
- `install.sh` — generates `config.toml` from `default_config.toml` instead of embedding hardcoded TOML
- `dev.sh benchmark` — now outputs `.md` report by default, `--live` for real-time table, `--csv` for raw data
- Dropped dependencies: `aho-corasick`, `num_cpus`
- Peak throughput: ~500k entries/s (C:\\), ~1.49M entries/s (C:\\Program Files) on i5-13400F

## v0.1.1 — 2026-02-20
- `benchmark.sh`, `build.sh`, `bump.sh` → `dev.sh`
- `uninstall.sh` absorbed into `install.sh --uninstall`
- ROADMAP and CHANGELOG split into separate files

## v0.1.0 — 2026-02-19 (Beta)
- Full code audit across all source files
- `main.rs` — extracted `build_matcher`, `build_search_config`, `resolve_dir`, `print_result`, `print_stats`
- `search.rs` — `MatchCtx` + `handle_match`, consolidated duplicate match+limit blocks
- `flags.rs` — `Default` trait, `flat_map` aliases, single-pass validation loop
- `config.rs` — removed `#[allow(dead_code)]`, cleaner `is_flag_available`, descriptions synced
- `display.rs` — help page trimmed, tip moved to top, functions extracted
- `install.sh` — update detection fixed (tags API), descriptions synced
- Scripts reorganized: `benchmark.sh`, `build.sh`, `bump.sh` → `dev.sh`
- `cargo clippy` zero warnings

## v0.0.8 — 2026-02-14
- README trimmed and restructured
- BENCHMARKS.md rebuilt with Linux data and Windows vs Linux comparison
- CONTRIBUTING.md cleaned up, script paths fixed
- `.gitattributes` added — LF/CRLF warnings resolved
- Help page trimmed — examples removed, tip moved to top
- `install.sh` update detection fixed (tags API)
- `bump.sh` version helper script added
- Scripts moved to `scripts/` directory
- Version strings fixed to use `env!("CARGO_PKG_VERSION")`
- Windows flag test pass — all flags and aliases verified

## v0.0.7 — 2026-02-12
- `--check` — validate config, print summary with warnings
- `--sync` — merge missing default flags without overwriting user config
- `--reset` — factory reset flags, preserve aliases and custom flags
- Dynamic `--help` showing user's aliases and custom flags
- `-L` limit race condition fixed (atomic overshoot clamped)
- Management flags section in help output

## v0.0.6 — 2026-02-10
- Custom flags support (`[custom]` in config.toml)
- Alias expansion before flag parsing
- `--exclude` flag for skipping directories
- `-v/--verbose` detailed stats (files + dirs separately)

## v0.0.5 — 2026-02-08
- Config-driven flag system — every flag remappable via config.toml
- `--config`, `--edit` management flags
- `-w/--where` with cd hint
- `-o/--open` with interactive picker for multiple results
- `-A/--all-drives` Windows multi-drive scan with per-drive breakdown

## v0.0.1 – v0.0.4 — 2026-02-05 to 2026-02-08
- Initial parallel file search with `ignore` crate walker
- AhoCorasick pattern matching
- `-e` extension filter, `-q` quiet mode, `-S` stats
- `-1/--first` early exit, thread scaling with `-t`
- Cross-platform installer and uninstaller
