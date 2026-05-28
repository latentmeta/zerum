# v0.2.0 crates.io release — ready when you are

## Done

- `cargo publish --dry-run --allow-dirty` passes
- `docs/RELEASING.md` publish checklist
- `Cargo.toml` exclude trims crate package (no tests/plans in tarball)
- CHANGELOG version links for 0.2.0

## Your steps

1. Commit all release + CI files (clean tree required for `cargo publish` without `--allow-dirty`)
2. `cargo publish --dry-run` (no `--allow-dirty`)
3. `git tag -a v0.2.0 -m "Zerum v0.2.0"`
4. `git push origin main && git push origin v0.2.0`
5. `cargo login` then `cargo publish`
6. GitHub Release binaries via `release.yml` on tag push
