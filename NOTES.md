# Zerum release notes (maintainer)

## v0.5.0 — ready to ship

- Versions → **0.5.0** (`Cargo.toml`, `pyproject.toml`, `Sastri.toml`, Homebrew formula)
- Feature: `--remediation-prompt` (grouped by type, severity-ordered)
- Docs: [RELEASE_v0.5.0.md](docs/RELEASE_v0.5.0.md), [DISTRO_CLOSURE_v0.5.0.md](docs/DISTRO_CLOSURE_v0.5.0.md)
- Branding: `docs/zerum-icon-01.png` + `docs/zerum-icon.jpg` in README

### Steps

1. Merge to `main`
2. Confirm PyPI Trusted Publisher (`publish-pypi.yml`, env `pypi`)
3. `cargo test && cargo clippy --all-targets --all-features -- -D warnings`
4. `cargo publish --dry-run`
5. Tag and push:
   ```bash
   git fetch upstream
   git tag -a v0.5.0 -m "Zerum v0.5.0 — remediation prompts for agents" upstream/main
   git push upstream v0.5.0
   ```
6. Wait for Release + Publish PyPI wheels
7. `cargo publish`
8. Update Homebrew sha256s; smoke `pip install zerum==0.5.0`

## Published

- **v0.4.2** — PyPI README absolute links
- **v0.4.1** — PyPI / Homebrew / distribution
- **v0.4.0** — profiles, Ruff, trust defaults
- **v0.2.0** — foundations catalog
