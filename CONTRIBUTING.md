# Contributing to ldx

Thanks for your interest in contributing! ldx is a fast, parallel file search CLI built on [parex](https://crates.io/crates/parex). Contributions of all kinds are welcome — bug fixes, features, benchmarks, documentation, and platform testing.

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
./scripts/dev.sh build
```

Or install directly:

```bash
./install.sh
```

See [README.md](README.md) for full installation instructions.

---

## Development Workflow

```bash
# Build and install locally
./scripts/dev.sh build

# Build in debug mode
./scripts/dev.sh build --debug

# Lint — must be clean before submitting
cargo clippy -- -D warnings

# Run tests
cargo test
```

Please ensure `cargo clippy -- -D warnings` produces zero warnings before opening a PR.

---

## Benchmarking

If you'd like to contribute benchmark results from your hardware, run:

```bash
# Warm cache (run anytime)
./scripts/dev.sh benchmark --runs 20 --warm

# Cold cache (run immediately after reboot, before anything else)
./scripts/dev.sh benchmark --runs 20 --cold

# With live table output
./scripts/dev.sh benchmark --runs 20 --live
```

Then open an issue or PR with the generated `.md` file attached. Please include your hardware specs (CPU, RAM, storage type). Results are credited in [BENCHMARKS.md](BENCHMARKS.md).

---

## Roadmap

Check [ROADMAP.md](ROADMAP.md) before starting work to avoid duplicating planned features. If you want to work on something not listed, open an issue first to discuss it.

---

## Pull Request Guidelines

- Keep PRs focused — one feature or fix per PR
- Run `cargo clippy -- -D warnings` and fix all warnings before submitting
- Add a clear description of what changed and why
- If adding a new flag, add it to `default_config.toml` and update the README flags table
- If you have benchmark results, include the `.md` output!

---

## Platform Testing

ldx supports Windows, Linux, and macOS. If you're on Linux or macOS, your benchmark results and bug reports are especially valuable — most development so far has been on Windows.

**Known platform notes:**
- Windows: tested on Windows 11, Git Bash required for scripts
- Linux: builds and runs, benchmarks not yet collected for v0.2.0
- macOS: builds and runs, benchmarks not yet collected
- HDD benchmarks: cold cache numbers on a full HDD welcome

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
