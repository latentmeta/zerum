# Releasing Zerum to crates.io

Checklist for publishing Zerum releases (current crate version in `Cargo.toml`).

## Pre-flight

1. **Version aligned**
   - `Cargo.toml` `version` matches the release tag (e.g. `0.4.0` / `v0.4.0`)
   - `CHANGELOG.md` has a dated section for that version
   - `docs/RELEASE_vX.Y.Z.md` matches the release (e.g. [RELEASE_v0.4.0.md](RELEASE_v0.4.0.md))

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
git fetch upstream
git show upstream/main:Cargo.toml | grep '^version'   # sanity check

git tag -a v0.4.0 -m "Zerum v0.4.0 — profiles and orchestration" upstream/main
git push upstream v0.4.0
cargo publish
```

- Tag **`upstream/main`** (or the exact release commit), not an old SHA. See [DISTRO_CLOSURE_v0.4.0.md](DISTRO_CLOSURE_v0.4.0.md).
- You cannot overwrite an existing version on crates.io once published.
- `cargo publish` uses the version in `Cargo.toml`; the git tag should match (`v0.4.0`).

## GitHub release binaries

Pushing tag `v*` triggers [`.github/workflows/release.yml`](../.github/workflows/release.yml), which builds Linux/macOS/Windows artifacts and attaches them to the GitHub Release.

**Requirements (or the workflow will not run):**

1. [`.github/workflows/release.yml`](../.github/workflows/release.yml) must already exist on the repo **default branch** (`main`) before you push the tag.
2. The tag must point at the commit you intend to ship (`Cargo.toml` version must match the tag, e.g. `0.2.0` for `v0.2.0`).
3. Push the tag to the **canonical** remote (`latentmeta/zerum`), not only a fork.

Verify after `git push upstream v0.2.0`:

```bash
gh run list --repo latentmeta/zerum --workflow=Release
gh release view v0.2.0 --repo latentmeta/zerum
```

Create the release notes body from `docs/RELEASE_vX.Y.Z.md` (or use `gh release create` manually; see below).

### Tag push did not create a release

Common causes:

| Cause | What to check |
|-------|----------------|
| Workflow missing on `main` when tag was pushed | `gh workflow list --repo latentmeta/zerum` — **Release** should exist; re-push tag after merge (see fix below). |
| Tag on wrong commit | `git show v0.2.0:Cargo.toml` — version must match tag. |
| Tag only on fork | `git ls-remote upstream refs/tags/v0.2.0` |
| Release workflow failed | `gh run list --repo latentmeta/zerum --workflow=Release` |

**Fix: move tag to the correct commit and re-push** (maintainer; coordinate if others already pulled the tag):

```bash
git fetch upstream
git show upstream/main:Cargo.toml | head -3   # confirm version

# Remove incorrect remote tag, then recreate on main tip
git push upstream :refs/tags/v0.2.0
git tag -d v0.2.0 2>/dev/null || true
git tag -a v0.2.0 -m "Zerum v0.2.0 — deterministic foundations" upstream/main
git push upstream v0.2.0
```

Wait for the **Release** workflow on [Actions](https://github.com/latentmeta/zerum/actions), then confirm assets on the GitHub Release page.

**Manual release (no binaries, or workflow blocked):**

```bash
gh release create v0.2.0 \
  --repo latentmeta/zerum \
  --title "Zerum v0.2.0" \
  --notes-file docs/RELEASE_v0.2.0.md
```

Upload binaries later from CI artifacts, or re-run after fixing the tag.

## After publish

1. Verify install: `cargo install zerum --version 0.2.0`
2. Smoke test: `zerum list-checks | head`, `zerum explain ZR001`
3. Open [crates.io/crates/zerum](https://crates.io/crates/zerum) and confirm metadata/README render correctly

## Package contents

`Cargo.toml` `exclude` keeps the published crate lean (no `tests/`, `plans/`, CI workflows). Shipped artifacts include:

- Library + `zerum` binary
- `README.md`, `CHANGELOG.md`, `LICENSE`, `zerum.toml.example`
- `docs/` (tutorials, release notes, coverage notes)
