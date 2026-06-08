# v0.4.0 distribution closure checklist

Use after [RELEASING.md](RELEASING.md). Full notes: [RELEASE_v0.4.0.md](RELEASE_v0.4.0.md).

## Pre-tag verification

```bash
git fetch upstream
git show upstream/main:Cargo.toml | grep '^version'   # must be 0.4.0
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo publish --dry-run
```

- [ ] `upstream/main` contains `.github/workflows/release.yml`
- [ ] [CHANGELOG.md](../CHANGELOG.md) has `[0.4.0]` dated section
- [ ] [docs/RELEASE_v0.4.0.md](RELEASE_v0.4.0.md) committed

## Tag (canonical remote)

**Tag must be on the commit that contains `version = "0.4.0"`**, not an older tag target.

```bash
git fetch upstream
git tag -d v0.4.0 2>/dev/null || true
git push upstream :refs/tags/v0.4.0 2>/dev/null || true   # only if replacing a bad tag

git tag -a v0.4.0 -m "Zerum v0.4.0 — profiles, Ruff orchestration, trust defaults" upstream/main
git push upstream v0.4.0
```

- [ ] `gh run list --repo latentmeta/zerum --workflow=Release` shows a run for `v0.4.0`
- [ ] GitHub Release has linux/macos/windows artifacts
- [ ] Release body pasted from [RELEASE_v0.4.0.md](RELEASE_v0.4.0.md)

If Release workflow did not run, see [RELEASING.md — Tag push did not create a release](RELEASING.md#tag-push-did-not-create-a-release).

## crates.io

```bash
cargo publish   # clean tree, no --allow-dirty
```

- [ ] `cargo install zerum --version 0.4.0`
- [ ] `zerum list-checks | wc -l` → 75
- [ ] [docs.rs/zerum](https://docs.rs/zerum/0.4.0) builds

## Smoke tests

```bash
zerum check tests/fixtures/clean_project              # exit 0
zerum check tests/fixtures/bad_project                # exit 1
zerum check tests/fixtures/simple_project --profile strict
zerum explain ZR001 | grep -v 'maintainability risk'  # curated text
```

Optional (if `ruff` installed):

```bash
zerum list-checkers
zerum check . --with-external ruff
```
