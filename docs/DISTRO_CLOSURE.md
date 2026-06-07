# v0.2.0 distribution closure checklist

Use this after [RELEASING.md](RELEASING.md) publish steps.

## Repository sync

- [ ] Open PR from `thanos/zerum` → `latentmeta/zerum` with CI workflows and release docs
- [ ] Merge to `latentmeta/zerum` `main`

## Tag and binaries

```bash
# Tag must be on the commit that contains version 0.2.0 in Cargo.toml (usually upstream/main)
git fetch upstream
git show upstream/main:Cargo.toml | grep '^version'
git tag -a v0.2.0 -m "Zerum v0.2.0 — deterministic foundations" upstream/main
git push upstream v0.2.0
```

If you already pushed a bad tag, see [RELEASING.md — Tag push did not create a release](RELEASING.md#tag-push-did-not-create-a-release).

- [ ] [release.yml](../.github/workflows/release.yml) completed on GitHub Actions (`gh run list --repo latentmeta/zerum --workflow=Release`)
- [ ] GitHub Release `v0.2.0` has linux/macos/windows artifacts
- [ ] Release notes body copied from [RELEASE_v0.2.0.md](RELEASE_v0.2.0.md)

## Smoke verification

```bash
cargo install zerum --version 0.2.0
zerum list-checks | wc -l    # expect 75
zerum explain ZR001
zerum check tests/fixtures/bad_project
```

- [ ] [crates.io/crates/zerum](https://crates.io/crates/zerum) metadata renders
- [ ] [docs.rs/zerum](https://docs.rs/zerum) builds and shows README + tutorials

## Post-publish housekeeping

- [ ] README crates.io badge and install one-liner
- [ ] [NOTES.md](../NOTES.md) archived or updated for post-publish state
- [ ] [plans/checklist-0.2.0.md](../plans/checklist-0.2.0.md) §1–2 reconciled with §8–9
