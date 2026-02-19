# Changelog

All notable changes to ldx are listed here. Newest first.

---

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
