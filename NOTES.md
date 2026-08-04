# Zerum release notes (maintainer)

## v0.4.1 — ready to ship

- Versions: `Cargo.toml` / `pyproject.toml` / `Sastri.toml` / Homebrew formula → **0.4.1**
- Docs: [docs/RELEASE_v0.4.1.md](docs/RELEASE_v0.4.1.md), [docs/DISTRO_CLOSURE_v0.4.1.md](docs/DISTRO_CLOSURE_v0.4.1.md)
- [CHANGELOG.md](CHANGELOG.md) `[0.4.1]` section
- Channels: crates.io + PyPI (Trusted Publishing) + Homebrew tap + GitHub Releases

### Your steps

1. Commit all v0.4.1 bumps and docs; merge to `main`
2. Confirm GitHub env `pypi` + PyPI Trusted Publisher (`publish-pypi.yml`)
3. `cargo test && cargo clippy --all-targets --all-features -- -D warnings`
4. `cargo publish --dry-run` (clean tree)
5. Tag **on `upstream/main`**:
   ```bash
   git fetch upstream
   git tag -a v0.4.1 -m "Zerum v0.4.1 — PyPI, Homebrew, distribution" upstream/main
   git push upstream v0.4.1
   ```
6. Wait for **Release** + **Publish PyPI wheels** → verify assets and PyPI
7. `cargo publish`
8. Fill Homebrew sha256s from `SHA256SUMS`; push `Formula/zerum.rb` to `latentmeta/homebrew-tap`
9. Smoke: `pip install zerum==0.4.1 && zerum --version`

## Published

- **v0.4.0** — GitHub Release / crates.io (profiles, Ruff, trust defaults)
- **v0.2.0** — crates.io (foundations catalog)
