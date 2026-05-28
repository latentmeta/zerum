# Releasing Zerum to crates.io

Checklist for publishing **v0.2.0** (and future versions).

## Pre-flight

1. **Version aligned**
   - `Cargo.toml` `version = "0.2.0"`
   - `CHANGELOG.md` has a dated `[0.2.0]` section
   - `docs/RELEASE_v0.2.0.md` matches the release

2. **Quality gates (local or CI)**

   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-targets --all-features
   ```

3. **Publish dry-run**

   ```bash
   cargo publish --dry-run
   ```

   Requires a **clean** git tree (commit or stash `NOTES.md`, `plans/`, etc.). Use `--allow-dirty` only to preview locally.

4. **Crates.io auth**

   ```bash
   # If publish fails with 403, unset a stale token that overrides login:
   unset CARGO_REGISTRY_TOKEN
   cargo login
   ```

## Publish to crates.io

From repo root, on `main` (or release branch) with all release files committed:

```bash
git tag -a v0.2.0 -m "Zerum v0.2.0 — deterministic foundations"
git push origin v0.2.0
cargo publish
```

- First publish of `0.2.0` uploads the crate; you cannot overwrite an existing version.
- `cargo publish` uses the version in `Cargo.toml`; tag name should match (`v0.2.0`).

## GitHub release binaries

Pushing tag `v*` triggers [`.github/workflows/release.yml`](../.github/workflows/release.yml), which builds Linux/macOS/Windows artifacts and attaches them to the GitHub Release.

Create the release notes body from `docs/RELEASE_v0.2.0.md` (or use `gh release create` after the workflow runs).

## After publish

1. Verify install: `cargo install zerum --version 0.2.0`
2. Smoke test: `zerum list-checks | head`, `zerum explain ZR001`
3. Open [crates.io/crates/zerum](https://crates.io/crates/zerum) and confirm metadata/README render correctly

## Package contents

`Cargo.toml` `exclude` keeps the published crate lean (no `tests/`, `plans/`, CI workflows). Shipped artifacts include:

- Library + `zerum` binary
- `README.md`, `CHANGELOG.md`, `LICENSE`, `zerum.toml.example`
- `docs/` (tutorials, release notes, coverage notes)
