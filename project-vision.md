# Project Vision: parex / ldx / parallax

**Last Updated:** 2026-02-21  
**Current Version:** v0.2.0 (ldx on parex engine) ✓ Shipped

---

## Core Mission
Build a blazing-fast parallel search framework (parex) with a clean CLI (ldx) and Spotlight-style GUI (parallax) wrappers. This is NOT about beating competitors — it's about creating perfectly tailored tools with complete control, config-driven everything, and benchmark-obsessed performance tuning.

---

## The Suite

### parex (the engine)
- **What:** Pure Rust parallel search framework, generic and embeddable
- **Not just files:** Can search anything traversable (posts, products, DB records, file systems)
- **Zero opinions:** No file-system logic in core — that's for wrappers
- **API:** `Source` + `Matcher` traits, `SearchBuilder` fluent API
- **Status:** v0.1.0 published to crates.io ✓

### ldx (CLI)
- **What:** File search CLI built on parex
- **Binary name:** `ldx` — permanent, no rename
- **Repo:** `localdex` (existing) — no repo rename needed
- **crates.io:** Not published — binary only, installed via install.sh
- **Philosophy:** Config-driven everything, zero hardcoded behavior
- **Status:** v0.2.0 — first version running on parex engine ✓

### parallax (GUI)
- **What:** Spotlight/Raycast style desktop app (Tauri + Rust)
- **UX:** Borderless, dynamic height, real-time streaming results (NO loading spinners)
- **UX note:** ~50ms debounce on first keystrokes when cold to avoid flicker on slow drives
- **Scope:** $HOME by default (not full drives — speed over completeness)
- **Plugins:** 5-tier system (CSS themes → Lua → JS → Python/compiled → Rust)
- **Status:** Mockup complete, development pending parex/ldx stable

### parafetch (future)
- **What:** neofetch/fastfetch alternative using parex for file counts
- **Unique angle:** Real-time scan stats, not static counts

### Enscribe (distant future)
- **What:** Cross-platform notes app for scripture/prayers/journaling
- **Status:** Standalone passion project, optional Parallax integration (keep optional)

---

## Naming — Final Decisions

- **parex** = parallel executor/explorer — crates.io library ✓
- **ldx** = CLI binary — permanent name, localdex repo stays as-is ✓
- **parallax** = GUI — new repo when the time comes

No CLI rename. `ldx` is already in users' PATH, has brand recognition, and conflicts with nothing.

---

## Version Scheme

```
0.0.X  → experimental (wild iteration)
0.X.X  → beta (polish, engine integration, GUI design)
r1.0   → stable release (battle-tested)
r1.X   → refinements
r2.0   → major leap (breakthroughs in speed/features)
```

The `r` prefix signals "production-ready."

---

## Current State (v0.2.0)

**Performance (i5-13400F, Windows 11, warm cache, 20 runs):**
- C:\Program Files (97k entries): **1,491,712 entries/s** @ t=16
- C:\Users\dylan (40k entries): **733,677 entries/s** @ t=12
- D:\ (40k entries): **702,785 entries/s** @ t=16
- C:\ (954k entries): **490,109 entries/s** @ t=12

**Architecture:**
- parex v0.1.0 published to crates.io ✓
- `DirectorySource` implements `parex::Source` using `ignore` crate parallel walker
- `search.rs` gutted — thin wrapper around `parex::search()` with ldx-specific matchers
- `config.rs` split → `config.rs` + `config_check.rs`
- `DEFAULT_CONFIG` extracted to `default_config.toml` + `include_str!`
- `flags.rs` refactored — `parse_args` split into focused helpers
- `install.sh` generates `config.toml` from `default_config.toml`
- `dev.sh benchmark` outputs `.md` report, `--live` table, `--csv` raw data
- Dropped deps: `aho-corasick`, `num_cpus`

**Features:**
- Config-driven flag parsing (users can remap any flag)
- Aliases (e.g., `ct = "-a -A -S -q --verbose"`)
- Custom flags (e.g., `-R` → auto-expand to `-e rs`)
- Management flags: `--config`, `--edit`, `--exclude`, `--check`, `--sync`, `--reset`
- Dynamic `--help` showing user's aliases and custom flags
- Multi-drive scanning (Windows)
- Thread scaling with auto-cap at logical cores via `std::thread::available_parallelism`

---

## Key Design Decisions

### Why Rust?
Speed, safety, embeddability. Perfect for both CLI and native library for Tauri/mobile.

### Why NOT indexing/MFT?
Real-time search fits the "instant discovery" UX better than stale indexes. Parallax searches on every keystroke — no waiting.

### Why config.toml over hardcoded flags?
Full user control. Want `-a` to mean something else? Change it. The binary is just a config executor.

### Why separate parex from ldx?
- Third parties can embed parex for non-file use cases
- Clean API surface, no file-system assumptions in core
- GUI and CLI share one engine, no duplicate logic

### Why Tauri over Electron?
Smaller binaries, native performance, Rust backend integration.

### Why NOT Lua-only plugins?
Accessibility. CSS themes need zero code. Python/JS covers most devs. Rust for power users.

---

## Roadmap

### v0.0.7 ✓ Shipped
- `--check`, `--sync`, `--reset` management commands
- Dynamic `--help` with user aliases
- `-L` limit race condition fix

### v0.0.8 ✓ Shipped
- README, BENCHMARKS.md, CONTRIBUTING.md updated
- Scripts moved to `scripts/`, `bump.sh` added
- Windows flag test pass, clippy clean

### v0.1.0 ✓ Shipped
- Full code audit — `flags.rs`, `main.rs`, `search.rs`, `config.rs`, `display.rs`
- Scripts consolidated: `benchmark.sh`, `build.sh`, `bump.sh` → `dev.sh`
- `uninstall.sh` absorbed into `install.sh --uninstall`
- ROADMAP/CHANGELOG split into separate files

### v0.2.0 ✓ Shipped
- parex v0.1.0 + v0.2.0 published to crates.io
- `DirectorySource` + parex integration
- `config.rs` split, `default_config.toml` extracted
- `flags.rs` refactored into focused helpers
- `install.sh` generates config from file
- `dev.sh benchmark` outputs `.md`, `--live`, `--csv`
- Dropped `aho-corasick`, `num_cpus`
- Parallax UI mockup complete (aesthetic + advanced modes)

### v0.3.0 — Test Suite (Next)
- Unit tests for `flags.rs` — alias expansion, custom flag resolution, validation combos
- Unit tests for `config.rs` — load, `is_flag_available`, path resolution
- Integration tests — full search round-trips using `DirectorySource`
- `benches/` with Criterion in parex — baseline throughput measurements
- parex: `collect_errors` stress test with permission-denied directories
- `--warn` flag — surface recoverable errors skipped during scan (permission denied, not found)
- GitHub Actions — automated pre-built binary releases on tag push (no manual release pages)

### v0.4.0 — parex v0.2.0
- True parallel matching — engine distributes match work across threads
- `--stale N` metadata filter — modified > N days ago, cheap pre-filter
- Async source support — `AsyncSource` trait for non-blocking IO
- parex v0.2.0 published to crates.io

### parallax — Future
- Tauri setup, borderless window prototype
- Real-time search integration with parex
- Settings panel (threads, scope, theme)
- Plugin system groundwork (Tier 1: CSS themes)
- Auto-benchmark on first launch → persist optimal config
- Progressive plugin tiers (Lua → JS → Python → Rust)

---

## Plugin System Architecture

### Tier 1 — Themes (CSS/JSON)
Zero code, just config files. Catppuccin, Nord, Dracula, Tokyo Night, Gruvbox.

### Tier 2 — Lightweight Scripts (Lua)
Simple data enrichment, CLI command triggers (`!`, `:`, `>`, `?`). ~200KB runtime.

### Tier 3 — Web Dev Friendly (JS/TS via Deno)
Familiar to most devs, moderate complexity plugins. Custom GUI allowed — JS devs expect to build UIs.

### Tier 4 — Power Integrations (Python/Go/C# via WASM or native)
External API calls (Steam, VirusTotal), heavy processing. Custom GUI allowed — e.g. Spotify sidebar, log panel.

### Tier 5 — Full System Access (Rust)
Replace backend parex if desired. Deepest API access. Custom GUI allowed.

**GUI modes:** Each plugin declares which modes it supports:
```toml
[plugin]
name = "spotify"
tier = 3
ui_modes = ["aesthetic", "advanced", "both"]
custom_gui = true
```
Tiers 1–2 provide CSS/data only — the shell renders them. Tiers 3–5 can own their pane entirely.

**Plugin priorities:** Weighting system so heavy plugins don't block light ones. UI shows "heavy plugin active" badge during execution.

---

## Non-Goals

- **No pre-indexing:** Real-time > stale indexes
- **No MFT reading on Windows:** Parallel traversal is fast enough, simpler code
- **No website (initially):** GitHub handles docs/downloads for free
- **No marketing:** Let quality speak
- **Not trying to beat ripgrep/fd/fzf:** Personal use first

---

## Session Notes

**v0.2.0 session (2026-02-21):**
- parex designed, built, tested, published to crates.io in one day
- `DirectorySource` using `build_parallel()` + `mpsc::channel` for true parallel streaming
- Initial `build()` attempt was ~4x slower — caught and fixed before shipping
- Peak: 1,491,712 entries/s on C:\Program Files @ t=16 (warm, 20 runs)
- Parallax mockup committed to localdex repo — dual mode (aesthetic + advanced)
- Accessibility-first design: Atkinson Hyperlegible, color accent picker, calm animations
- `dev.sh benchmark` overhauled — `.md` output by default, `--live` flag for real-time table

**End Vision:** A suite of tools so fast, so clean, so configurable that they become daily drivers — not because they beat the competition, but because they're exactly what we need.

🦀
