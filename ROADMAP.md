# Roadmap

Current status: **v0.1.0 Beta** — engine separation next.

---

## v0.1.1 — Script Consolidation (Next)
- `benchmark.sh`, `build.sh`, `bump.sh` → `dev.sh` ✓
- `uninstall.sh` absorbed into `install.sh --uninstall` ✓
- Fix `--uninstall` header ordering in install.sh

## Engine Separation Milestone
- Create `parex` repo — extract core, design Query API
- Publish `parex` v0.1.0 to crates.io
- Create `prx` repo — rename localdex, depend on `parex`
- Publish `prx` v0.1.0
- Unit tests (much easier post-separation)

## parex Query API
- Builder pattern: `parex::search().matching("x").in_dir("~").run()`
- Typed errors with `thiserror`
- Opt-in error collection via `.collect_errors(true)`
- `--stale N` metadata filter (modified > N days ago) — cheap pre-filter before parallel walk

## parallax (GUI)
- Tauri + Rust backend, borderless window
- Real-time streaming results, no loading spinners
- Plugin system: CSS themes → Lua → JS → Python → Rust
- Auto-benchmark on first launch

## Future
- `parafetch` — neofetch alternative using parex for file counts
- macOS benchmarks
- HDD benchmark contributions

---

See [CHANGELOG.md](CHANGELOG.md) for shipped history.
