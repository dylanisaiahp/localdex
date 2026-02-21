# Project Vision: parex / ldx / parallax

**Last Updated:** 2026-02-21  
**Current Version:** ldx v0.2.0 · parex v0.2.1

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
- **Status:** v0.2.1 published to crates.io ✓

### ldx (CLI)
- **What:** File search CLI built on parex
- **Binary name:** `ldx` — permanent, no rename
- **Repo:** `localdex` (existing) — no repo rename needed
- **crates.io:** Not published — binary only, installed via install.sh
- **Philosophy:** Config-driven everything, zero hardcoded behavior
- **Status:** v0.2.0 — running on parex engine ✓

### parallax (GUI)
- **What:** Spotlight/Raycast style desktop app (Tauri + Svelte)
- **UX:** Borderless, dynamic height, real-time streaming results (NO loading spinners)
- **UX note:** ~50ms debounce on first keystrokes when cold to avoid flicker on slow drives
- **Scope:** $HOME by default (not full drives — speed over completeness)
- **Default accent:** Blue — user-configurable, ~30 shipped colors, plugin-extensible
- **Plugins:** 5-tier system (CSS themes → Lua → JS → Python → Rust)
- **Status:** Mockup complete (`parallax-mockup.html` in localdex repo), development pending ldx/parex ~0.5.0

### parafetch (future)
- **What:** neofetch/fastfetch alternative using parex for file counts
- **Unique angle:** Real-time scan stats, not static counts

### Enscribe (distant future)
- **What:** Cross-platform notes app for scripture/prayers/journaling
- **Status:** Standalone passion project (currently Flutter), Rust/Tauri rewrite planned post-parallax

---

## Naming — Final Decisions

- **parex** = parallel executor/explorer — crates.io library ✓
- **ldx** = CLI binary — permanent name, localdex repo stays as-is ✓
- **parallax** = GUI — new repo when the time comes

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

## Current State

**Performance (i5-13400F, Windows 11, warm cache, 20 runs):**
- C:\Program Files (97k entries): **1,491,712 entries/s** @ t=16
- C:\Users\dylan (40k entries): **733,677 entries/s** @ t=12
- D:\ (40k entries): **702,785 entries/s** @ t=16
- C:\ (954k entries): **490,109 entries/s** @ t=12

**parex v0.2.1:**
- `Source` + `Matcher` traits, `SearchBuilder` fluent API
- `#[non_exhaustive]` errors, `is_recoverable()` / `is_fatal()`
- `source_err()` / `matcher_err()` convenience constructors
- `#![forbid(unsafe_code)]`
- 330 SLoC · 7 integration tests · 7 doc tests
- Full docs: `README.md`, `DOCS.md`, `CHANGELOG.md`, `CONTRIBUTING.md`

**ldx v0.2.0:**
- `DirectorySource` implements `parex::Source` using `ignore` crate parallel walker
- Config-driven flag parsing, aliases, custom flags
- Management flags: `--check`, `--sync`, `--reset`, `--edit`, `--config`
- `dev.sh` — build, benchmark (`--live`, `--csv`), bump
- `install.sh` — generates config from `default_config.toml`, preserves aliases on rebuild

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

### Why Svelte for parallax frontend?
Small bundle, compiles away at build time, close to plain HTML/CSS/JS, great Tauri ecosystem fit.

### Why NOT Lua-only plugins?
Accessibility. CSS themes need zero code. Python/JS covers most devs. Rust for power users.

---

## Roadmap

### ldx

#### v0.0.7 ✓ Shipped
- `--check`, `--sync`, `--reset` management commands
- Dynamic `--help` with user aliases
- `-L` limit race condition fix

#### v0.0.8 ✓ Shipped
- README, BENCHMARKS.md, CONTRIBUTING.md updated
- Scripts moved to `scripts/`, `bump.sh` added
- Windows flag test pass, clippy clean

#### v0.1.0 ✓ Shipped
- Full code audit — `flags.rs`, `main.rs`, `search.rs`, `config.rs`, `display.rs`
- Scripts consolidated → `dev.sh`
- ROADMAP/CHANGELOG split

#### v0.2.0 ✓ Shipped
- parex integration, `DirectorySource`
- `config.rs` split, `default_config.toml` extracted
- `install.sh` alias-safe rebuild
- `dev.sh benchmark` `.md` output, `--live`, `--csv`
- Parallax mockup complete

#### v0.3.0 — Test Suite (Next)
- Unit tests: `flags.rs`, `config.rs`
- Integration tests: full search round-trips via `DirectorySource`
- `--warn` flag — surface recoverable errors skipped during scan
- GitHub Actions — automated pre-built binary releases on tag push

#### v0.4.0
- Linux benchmark pass (parex-powered numbers)
- README polish, ldx link in parex docs

#### v0.5.0 — Pre-Parallax Stable
- Final cleanup, any outstanding minor items
- → Parallax development begins

---

### parex

#### v0.1.0 ✓ Shipped
- Initial release, core traits, builder API, error type

#### v0.2.0 ✓ Shipped
- `#[non_exhaustive]`, `Box<dyn Error>` variants, `NotFound` recoverable
- `is_fatal()`, `source_err()`, `matcher_err()`
- Dropped `ignore` dep

#### v0.2.1 ✓ Shipped
- `DOCS.md`, `CHANGELOG.md`, `CONTRIBUTING.md`
- README updated — `cargo add parex`, ordering guarantees, `PAREX_DESIGN.md` removed

#### v0.3.0 — Test Suite (Next)
- Criterion benchmarks in `benches/`
- `collect_errors` stress test with permission-denied directories
- Additional edge case tests — limits, thread counts, empty sources

#### v0.4.0
- Async source support — `AsyncSource` trait for non-blocking IO
- True parallel matching — engine distributes match work across threads
- `--stale N` metadata filter groundwork

#### v0.5.0 — Pre-Parallax Stable
- Workspace setup groundwork (`parex-fs` etc.) if adoption warrants it
- API review before parallax locks in dependency

---

### parallax (Future — begins after ldx + parex ~0.5.0)

```
v0.1.0 — Tauri skeleton, borderless window, parex search, CSS themes (Tier 1)
v0.2.0 — Lua plugins (Tier 2), settings panel, accent colors (~30 shipped)
v0.3.0 — JS/TS via Deno (Tier 3), custom GUI plugins, Triggers system
v0.4.0 — Python (Tier 4), Go (v0.4.5)
v0.5.0 — Rust plugins (Tier 5), permissions system, plugin marketplace groundwork
```

---

## Plugin System Architecture

### Tier 1 — Themes (CSS/JSON)
Zero code, just config files. Ships with ~30 accent colors. Catppuccin, Nord, Dracula, Tokyo Night, Gruvbox and more.

### Tier 2 — Lightweight Scripts (Lua)
Simple data enrichment. ~200KB runtime.

### Tier 3 — Web Dev Friendly (JS/TS via Deno)
Custom GUI allowed. Familiar to most devs.

### Tier 4 — Power Integrations (Python, Go)
Custom GUI allowed. External API calls, heavy processing. Python v0.4.0, Go v0.4.5.

### Tier 5 — Full System Access (Rust)
Custom GUI allowed. Replace backend parex if desired. Deepest API access.

**Plugin manifest:**
```toml
[plugin]
name = "spotify"
tier = 3
ui_modes = ["aesthetic", "advanced", "both"]
custom_gui = true
```

**Permissions system:** Each plugin declares permissions at install time — network access, clipboard, filesystem, system commands. Tiers 1–2 cannot request network access.

**Official plugins badge:** Official plugins show `by Parallax ✓`. Community plugins show author name only.

**Triggers system:** `!g search` → Google, `> cmd` → run command. POC ships with parallax, community-extensible via plugins.

**Plugin priorities:** Weighting system so heavy plugins don't block light ones. UI shows "heavy plugin active" badge.

---

## Non-Goals

- **No pre-indexing:** Real-time > stale indexes
- **No MFT reading on Windows:** Parallel traversal is fast enough, simpler code
- **No website (initially):** GitHub handles docs/downloads for free
- **No marketing:** Let quality speak
- **Not trying to beat ripgrep/fd/fzf:** Personal use first
- **No companion app (for now):** Could be a Tier 4-5 parallax plugin first, standalone app if adoption warrants it

---

## Session Notes

**Week 1 (2026-02-14 to 2026-02-21):**
- ldx pushed to GitHub on day 2
- parex designed, built, tested, and published to crates.io in one session
- `DirectorySource` using `build_parallel()` + `mpsc::channel` — initial `build()` was ~4x slower, caught and fixed before shipping
- Peak: 1,491,712 entries/s on C:\Program Files @ t=16 (warm, 20 runs)
- Parallax mockup complete — dual mode (aesthetic + advanced), Atkinson Hyperlegible, accent color picker
- parex v0.2.1 — full docs suite shipped, PAREX_DESIGN.md removed
- Reddit post: 3.1K views, 22 crates.io downloads in first 12 hours
- One project at a time rule established — avoids context confusion across repos

**End Vision:** A suite of tools so fast, so clean, so configurable that they become daily drivers — not because they beat the competition, but because they're exactly what we need.

🦀
