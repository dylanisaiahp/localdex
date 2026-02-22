# Visual Companion Tool – Design & Implementation Overview

**Project Name (working):** Visual Companion / VisComp / PreviewHub (bike-shed later)  
**Repo Idea:** [github.com/dylanisaiahp/visual-companion](https://github.com/dylanisaiahp/visual-companion) (or integrate into an existing one)  
**Current Date:** February 20, 2026  
**Status:** Conceptual / Pre-MVP brainstorming  

---

## Core Vision & Purpose

A lightweight, privacy-first, cross-platform desktop companion app that provides instant visual feedback and context for files you're editing in a fast external editor (e.g., Zed, Helix, Neovim, VS Code minimal fork).

**Primary users:**

- Developers (rapid UI/GUI iteration, docs writing, data preview)  
- Writers / docs folks (real-time Markdown/PDF renders)  
- Everyday users (quick multi-PDF/Excel/image viewer)  

**Philosophy:**

- Zero telemetry, fully local processing  
- Ultra-fast launch (sub-second, ideally <500 ms)  
- Maximal configurability: hide/collapse any pane → "dumb it down" to a simple viewer  
- No built-in code editor — pair with Zed or similar for editing  
- Focus: see what you're building/reading instantly, across formats and simulated platforms  

**Differentiator vs existing tools:**

| Tool | Limitation |
|------|------------|
| VS Code / Cursor / Zed extensions | Tied to one editor |
| Figma / Lunacy | Design-first, not code-aware |
| Obsidian / Typora | Markdown-centric |

**Visual Companion** is editor-agnostic, multi-format, cross-platform preview + visualization hub.

---

## High-Level Architecture

**Stack:** Tauri 2.x (Rust backend + Web frontend)  

**Why Tauri:**

- Native performance (startup <500 ms, idle RAM ~30–80 MB, binary ~5–15 MB)  
- Cross-platform consistency (Windows, macOS, Linux)  
- Embedded WebView for renders (WebView2 / WebKitGTK / WKWebView)  

**Architecture Flow:**

    [System File Watcher / notify-rs]
              ↓ (on save / debounce)
    [Tauri Rust Backend]
    - File parsing (calamine for XLSX, pulldown-cmark/mdbook for MD, pdf crate bindings)
    - Git repo introspection (git2-rs)
    - Script execution (std::process)
              ↓ (IPC / invoke)
    [Frontend (Svelte / vanilla JS / Leptos?)]
    - Resizable/collapsible panes
    - Center: dynamic preview renderer
    - Left: file tree + interactive git viz
    - Right: tall console + script buttons

---

## Default Layout & "Dumb-Down" System

**Default (Dev Mode) – Balanced for coders/docs writers:**

- **Left sidebar (~20–25% width, resizable/collapsible):**  
  - Top: File explorer/tree (project folders, drag-drop support)  
  - Bottom: Interactive Git commit breakdown (timeline/graph, selection-linked color highlights)  

- **Center (~60%):** Main preview pane  
  - Tabbed or tiled multi-view (Markdown render, PDF, Excel grid, web responsive)  
  - Dynamic modes: web/mobile/desktop/Linux DE simulations  

- **Right sidebar (~15–20%, tall):** Console/output + toolbar of script buttons (build.sh, benchmark.sh, etc.)  

**Dumb-Down / Hide Mechanism:**

- Every pane has toggle icon / shortcut (e.g., `Ctrl+Shift+L/R/C`)  
- Profiles via dropdown/hotkey: "Preview Only", "Compare Grid", "Reader Mode"  
- Startup defaults to center-only (full preview + slim top bar for open/mode)  
- Drag-resize, float/undock panes for multi-monitor  
- Goal: Non-dev opens app → drag PDF/Excel → sees clean viewer, no dev clutter  

---

## Key Features & Prioritization

### MVP (Phase 1 – Get something usable fast)

- Instant launch + file watching (`notify-rs`)  
- Center preview for:  
  - Markdown (GFM + Mermaid/KaTeX via marked or custom renderer)  
  - PDF (basic multi-page viewer, e.g., via pdf.js in WebView)  
  - Excel (.xlsx read-only grid via calamine + web table component like TanStack Table)  
- Hideable left/right panes → simple multi-file viewer  
- Basic file tree (left) for folder browsing  

### Phase 2 – Dev Power Features

- Interactive Git viz (commit timeline, selection → highlight commits touching file/line)  
- Script buttons (auto-discover `.sh` / `package.json` scripts, run → tall console output)  
- Real-time sync: Zed save → preview refresh (file watcher + debounce)  

### Phase 3 – Dynamic / Cross-Platform Previews

- Web responsive: multiple device frames/breakpoints in grid  
- Desktop: simulated window chrome (Linux DE themes: GNOME/KDE/etc. via CSS)  
- Mobile emulation (for PWAs/web)  
- Scroll/cursor sync approximations  

### Phase 4 – Polish & Extensions

- Accessibility auditor overlays  
- Local LLM bridge (Ollama) for suggestions in preview  
- Export rendered views (HTML/PDF/screenshots)  
- Multi-format compare mode (before/after, cross-platform diffs)  

---

## Tech Stack Choices (Why?)

- **Tauri + Rust:** Sub-second startup, tiny size, native feel, no Electron bloat  
- **Frontend:** Svelte (reactive, small bundle) or Leptos (Rust-fullstack if ambitious)  
- **Parsing crates:**  
  - Markdown: pulldown-cmark + custom renderer  
  - XLSX: calamine (fast reader) or umya-spreadsheet (if write needed later)  
  - PDF: pdf crate / printpdf (generation) or pdf.js (WebView render for preview)  
- **Git:** git2-rs for commit history parsing  
- **Watching:** notify-rs (cross-platform fs events)  

---

## Non-Goals (to stay lean)

- No built-in editor (use Zed)  
- No cloud features / telemetry  
- No heavy indexing (on-the-fly parsing)  
- No full editing of files (preview + annotate only)  

---

## Why Build This?

Existing tools force trade-offs:

- **Bloated IDEs (VS Code)** → privacy + performance hit  
- **Editor-specific previews (Zed Markdown pane)** → limited formats  
- **Heavy viewers (Adobe Acrobat)** → overkill for quick glances  

**This fills the gap:** fast companion window that lives beside your editor, shows polished renders instantly, hides complexity when not needed, and scales from dev prototyping to everyday document glancing.  

---

**Next Steps:** expand with sketches, backlog tickets, or prototypes. MVP could be a weekend Tauri skeleton + Markdown/PDF viewer to validate launch speed and feel.