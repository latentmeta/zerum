# Zerum release notes (maintainer)

## v0.4.0 — ready to ship

- Code on `main`: `Cargo.toml` **0.4.0**, commit includes profiles + Ruff + AST batch
- Docs: [docs/RELEASE_v0.4.0.md](docs/RELEASE_v0.4.0.md), [docs/DISTRO_CLOSURE_v0.4.0.md](docs/DISTRO_CLOSURE_v0.4.0.md)
- [CHANGELOG.md](CHANGELOG.md) `[0.4.0]` section

### Your steps

1. Commit any pending doc updates (RELEASE_v0.4.0, RELEASING, README)
2. `cargo test && cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo publish --dry-run` (clean tree)
4. Tag **on `upstream/main`** (must have `version = 0.4.0`):
   ```bash
   git fetch upstream
   git tag -a v0.4.0 -m "Zerum v0.4.0" upstream/main
   git push upstream v0.4.0
   ```
5. Wait for Release workflow → verify GitHub Release assets
6. `cargo publish`
7. Smoke: `cargo install zerum --version 0.4.0`

### v0.2.0 tag note

If `v0.2.0` tag still points at wrong commit (`Cargo.toml` 0.1.0), fix per [docs/RELEASING.md](docs/RELEASING.md) before relying on that release page.

## Published

- **v0.2.0** — crates.io (foundations catalog)
