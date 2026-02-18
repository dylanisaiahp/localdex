# Contributing to ldx

Thanks for your interest in contributing! ldx is a fast, parallel file search CLI written in Rust. Contributions of all kinds are welcome — bug fixes, features, benchmarks, documentation, and platform testing.

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs) (stable)
- [Git](https://git-scm.com/)
- Windows: [Git Bash](https://gitforwindows.org/) for running `.sh` scripts

### Build & Install

```bash
git clone https://github.com/dylanisaiahp/localdex
cd localdex
./scripts/build.sh
```

Or install directly:

```bash
./scripts/install.sh
```

See [README.md](README.md) for full installation instructions.

---

## Development Workflow

```bash
# Build in debug mode
./scripts/build.sh --debug

# Build in release mode
./scripts/build.sh --release

# Run clippy before submitting
cargo clippy

# Run tests
cargo test
```

Please ensure `cargo clippy` produces no warnings before opening a PR.

---

## Benchmarking

If you'd like to contribute benchmark results from your hardware, run:

```bash
# Warm cache (run anytime)
./scripts/benchmark.sh --runs 20 --warm

# Cold cache (run immediately after reboot, before anything else)
./scripts/benchmark.sh --runs 20 --cold
```

Then open an issue or PR with your CSV files attached. Please include your hardware specs (CPU, RAM, storage type). Results are stored in `/benchmarks` and credited in the README.

---

## Roadmap

Check the [README roadmap](README.md#roadmap) before starting work to avoid duplicating planned features. If you want to work on something not listed, open an issue first to discuss it.

Current focus is **v0.1.0 Beta** — code audit, unit tests, and Linux benchmarks. See the roadmap in README.md for details.

---

## Pull Request Guidelines

- Keep PRs focused — one feature or fix per PR
- Run `cargo clippy` and fix all warnings before submitting
- Add a clear description of what changed and why
- If adding a new flag, add it to `DEFAULT_CONFIG` in `main.rs` and update the README flags table
- If you have benchmark results, include them!

---

## Platform Testing

ldx aims to support Windows, Linux, and macOS. If you're on Linux or macOS, your benchmark results and bug reports are especially valuable — most development so far has been on Windows.

**Known platform notes:**
- Windows: tested on Windows 11, Git Bash required for scripts
- Linux/macOS: builds and runs but benchmarks not yet collected
- HDD benchmarks: cold cache numbers on a full HDD welcome — current HDD data was from a nearly empty drive

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
