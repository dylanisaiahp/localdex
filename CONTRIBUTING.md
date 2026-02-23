# Contributing to ldx

Thanks for your interest in contributing! ldx is a fast, parallel file search CLI built on [parex](https://crates.io/crates/parex). Contributions of all kinds are welcome — bug fixes, features, benchmarks, documentation, and platform testing.

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs) (stable)
- [Git](https://git-scm.com/)
- Windows: [Git Bash](https://gitforwindows.org/) for running the install script

### Build & Install

```bash
curl -sSf https://raw.githubusercontent.com/dylanisaiahp/localdex/main/scripts/install.sh | sh
```

Or clone and build manually:

```bash
git clone https://github.com/dylanisaiahp/localdex
cd localdex
cargo build --release
```

---

## Development Workflow

```bash
# Build release binary
cargo build --release

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
ldx bench --runs 10 --live
```

Then open an issue or PR with the generated `.md` file attached. Please include your hardware specs (CPU, RAM, storage type).

---

## Pull Request Guidelines

- Keep PRs focused — one feature or fix per PR
- Run `cargo clippy -- -D warnings` and fix all warnings before submitting
- Add a clear description of what changed and why
- If adding a new flag, add it to `default_config.toml` and update the README flags table
- If you have benchmark results, include the `.md` output!

---

## Platform Testing

ldx supports Windows, Linux, and macOS. If you're on Linux or macOS, your benchmark results and bug reports are especially welcome.

**Known platform notes:**
- Windows: tested on Windows 11, Git Bash required for install script
- Linux: actively developed and tested on CachyOS
- macOS: builds and runs, benchmarks not yet collected

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
