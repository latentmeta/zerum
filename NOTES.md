# Zerum release notes (maintainer)

## v0.4.2 — ready to ship

Docs-only: absolute GitHub links in README for PyPI. Versions → **0.4.2**.

- [docs/RELEASE_v0.4.2.md](docs/RELEASE_v0.4.2.md)
- [docs/DISTRO_CLOSURE_v0.4.2.md](docs/DISTRO_CLOSURE_v0.4.2.md)

### Steps

1. Merge to `main`
2. Tag and push `v0.4.2`
3. Wait for Release + Publish PyPI wheels
4. `cargo publish`
5. Optional: yank PyPI `0.4.1` (docs-only; not required)
6. Smoke: `pip install zerum==0.4.2`

## Published

- **v0.4.1** — PyPI / GitHub Release / crates.io (distribution)
- **v0.4.0** — profiles, Ruff, trust defaults
- **v0.2.0** — foundations catalog
