# Project Vision: parex / prx / parallax

**Last Updated:** 2026-02-19  
**Current Version:** v0.1.0 Beta

---

## Core Mission
Build a blazing-fast parallel search framework (parex) with clean CLI (prx) and Spotlight-style GUI (parallax) wrappers. This is NOT about beating competitors—it's about creating perfectly tailored tools with complete control, config-driven everything, and benchmark-obsessed performance tuning.

---

## The Suite

### parex (the engine)
- **What:** Pure Rust parallel search framework, generic and embeddable
- **Not just files:** Can search anything traversable (posts, products, DB records, file systems)
- **Zero opinions:** No file-system logic in core—that's for wrappers
- **API design:** Simple Query struct in, Results out (zero overhead)
- **Launch:** v0.1.0 on crates.io after separation

### prx (CLI)
- **What:** File search wrapper around parex
- **Binary name:** `ldx` (transitional), then `prx`
- **Note:** No crate conflict on crates.io, but a CLI called PROJAX uses the `prx` command — verify before final rename or keep `ldx` as the permanent binary name
- **Philosophy:** Config-driven everything, zero hardcoded behavior
- **Launch:** v0.1.0 after rename from localdex

### parallax (GUI)
- **What:** Spotlight/Raycast style desktop app (Tauri + Rust)
- **UX:** Borderless, dynamic height, real-time streaming results (NO loading spinners)
- **UX note:** Consider a ~50ms debounce/throttle on first keystrokes when cold to avoid flicker on slow drives
- **Scope:** $HOME by default (not full drives—speed over completeness)
- **Plugins:** 5-tier system (CSS themes → Lua → JS → Python/compiled → Rust)
- **Launch:** After parex/prx stable

### parafetch (future)
- **What:** neofetch/fastfetch alternative using parex for file counts
- **Unique angle:** Real-time scan stats, not static counts

### Enscribe (distant future)
- **What:** Cross-platform notes app for scripture/prayers/journaling
- **Status:** Standalone passion project, optional Parallax integration (keep optional)

---

## Name Changes (Pending Engine Separation)

**Why rename?** "localdex" no longer fits—it's not just local files anymore.

- **parex** = parallel executor/explorer (available on crates.io ✓)
- **prx** = CLI wrapper (short, punchy) — verify no `prx` command conflict first
- **parallax** = GUI (sleek, visual metaphor for depth perception)

**Timing:** Rename during engine separation milestone, not before.

---

## Version Scheme

```
0.0.X  → experimental (wild iteration)
0.X.X  → beta (code polish, separation prep, GUI design)
r1.0   → stable release (battle-tested)
r1.X   → refinements
r2.0   → major leap (breakthroughs in speed/features)
```

The `r` prefix signals "production-ready." Year-based versioning was considered but rejected—too complex for minimal benefit.

---

## Current State (v0.0.8)

**Performance:**
- Windows peak: 1,641,700 entries/s @ 16t (i5-13400F, C:\Program Files)
- Windows sustained: 526,404 entries/s @ 10t on 945k files (C:\ drive)
- Linux peak: 7,065,858 entries/s @ 16t (Ryzen 7 5825U, CachyOS NVMe)
- Linux sustained: 6,026,774 entries/s @ 16t on home dir
- Linux cold cache faster than Windows warm cache (56x difference on full drive)
- Desktop CachyOS benchmarks pending

**Features:**
- Config-driven flag parsing (users can remap any flag)
- Aliases (e.g., `repo = "localdex -D -d D: -1 -S -w -q"`)
- Custom flags (e.g., `-P` → auto-expand to `-e pdf`)
- Management flags: `--config`, `--edit`, `--exclude`, `--check`, `--sync`, `--reset`
- Dynamic `--help` showing user's aliases and custom flags
- Version sourced from `Cargo.toml` via `env!("CARGO_PKG_VERSION")`
- `-L` limit race condition fixed (clamped reported count, early-exit guard)
- Cross-platform installer/uninstaller with source cleanup options
- Modular codebase: config.rs, flags.rs, search.rs, display.rs, launcher.rs, main.rs

**What works:**
- File and directory search with substring/extension matching
- Multi-drive scanning (Windows)
- Thread scaling with auto-cap at logical cores
- Real-time result streaming with `-1`/`-L` limits
- `-w/--where` with cd hints
- `-o/--open` with picker for multiple results
- `--check` validates config, catches duplicate flags and missing targets
- `--sync` adds missing default flags without touching user aliases/custom flags
- `--reset` restores default flags, preserves `[aliases]`, `[custom]`, `[meta]`

---

## Key Design Decisions

### Why Rust?
Speed, safety, embeddability. Perfect for both CLI and native library for Tauri/mobile.

### Why NOT indexing/MFT?
Real-time search fits the "instant discovery" UX better than stale indexes. Parallax searches on every keystroke—no waiting.

### Why config.toml over hardcoded flags?
Full user control. Want `-a` to mean something else? Change it. Want `pdf` as an alias? Add it. The binary is just a config executor.

### Why drop pico_args?
Smaller binary, zero parsing overhead, full control over dynamic flag names from config.

### Why separate parex from prx?
- Third parties can embed parex for non-file use cases (e.g., X searching millions of posts)
- Clean API surface, no file-system assumptions in core
- GUI and CLI share one engine, no duplicate logic

### Why Tauri over Electron?
Smaller binaries, native performance, Rust backend integration. No bloated Chromium runtime.

### Why NOT Lua-only plugins?
Accessibility. CSS themes need zero code. Python/JS covers most devs. Rust for power users. Tiered system = broader adoption.

---

## Non-Goals

- **No pre-indexing:** Real-time > stale indexes
- **No MFT reading on Windows:** Parallel traversal is fast enough, simpler code
- **No website (initially):** GitHub handles docs/downloads for free
- **No marketing:** Let quality speak. If it's good, people will find it.
- **Not trying to beat neofetch/ripgrep/fd/fzf:** This is for personal use first. Others benefit if they want.

---

## Why NOT X?

### GSX/ASX specialized tools?
Unnecessary. parex already does universal parallel search. One tool, any data source.

### `--goto` flag?
Shell limitation. Child process can't `cd` the parent. Use `-w/--where` + manual cd instead.

### Year-based versioning (v26.X)?
Too complex, minimal benefit. Clean semantic versioning is simpler and familiar.

---

## Mobile Vision (Distant Future)

- **Enscribe mobile:** Cross-platform notes app companion
- **File browser:** parex-powered Android/iOS file search
- **Technology:** React Native or Flutter with parex compiled as native library

Not a priority until parex/prx/parallax/parafetch are stable.

---

## Current Challenges

- **Testing rigor:** Need comprehensive flag testing before each release (v0.0.5 shipped with broken `--help`, v0.0.7 caught `-L` race condition in testing)
- **Documentation debt:** Need parex API docs, plugin dev guides, theme creation tutorials
- **prx name conflict:** Verify PROJAX's `prx` command collision before final rename

---

**End Vision:** A suite of tools so fast, so clean, so configurable that they become daily drivers—not because they beat the competition, but because they're exactly what we need.

🦀
