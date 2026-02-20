# Roadmap

Current status: **v0.2.0** — parex integrated, engine overhaul complete.

---

## v0.2.0 — Engine Overhaul ✓
- `parex v0.1.0` published to crates.io ✓
- `DirectorySource` implemented in ldx ✓
- `search.rs` gutted — replaced with `parex::search()` wrapper ✓
- `config.rs` split → `config.rs` + `config_check.rs` ✓
- `default_config.toml` extracted from source ✓
- `flags.rs` refactored into focused helpers ✓
- `install.sh` generates config from file ✓
- `dev.sh benchmark` outputs `.md` report, `--live` table, `--csv` raw data ✓
- Dropped `aho-corasick`, `num_cpus` ✓

## v0.3.0 — Test Suite
- Unit tests for `flags.rs` — alias expansion, custom flag resolution, validation combos
- Unit tests for `config.rs` — load, is_flag_available, path resolution
- Integration tests — full search round-trips using `DirectorySource`
- `benches/` with Criterion in parex — baseline throughput measurements
- parex: `collect_errors` stress test with permission-denied directories

## v0.4.0 — parex v0.2.0 (Parallelism Improvements)
- True parallel matching — engine distributes match work across threads
- Async source support — `AsyncSource` trait for non-blocking IO
- `--stale N` metadata filter — modified > N days ago, cheap pre-filter
- parex publish v0.2.0 to crates.io

## parallax (GUI) — Future
- Tauri + Rust backend, borderless window
- Real-time streaming results
- Dual mode: aesthetic (Atkinson Hyperlegible) and advanced (JetBrains Mono)
- Plugin system: CSS themes → Lua → JS → Python → Rust
- Color accent picker, accessibility-first design

## Future
- `parafetch` — neofetch alternative using parex for file counts
- macOS benchmarks and CI
- Pre-built binary releases
- HDD benchmark contributions

---

See [CHANGELOG.md](CHANGELOG.md) for shipped history.
