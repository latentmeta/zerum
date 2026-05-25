# Releasing to crates.io

Project repository: [github.com/latentmeta/zerum](https://github.com/latentmeta/zerum)

Ensure `repository` and `homepage` in `Cargo.toml` match that URL before publishing.

## Prerequisites

1. [crates.io](https://crates.io/) account (log in with GitHub).
2. API token: Account Settings → Create New Token.
3. One-time login on your machine:

```bash
cargo login
# paste token when prompted
```

## Pre-publish checklist

```bash
cargo test
cargo clippy -- -D warnings
cargo publish --dry-run
```

Confirm `zerum.toml.example` is in the package (`cargo package --list | grep zerum.toml`).

## Publish

```bash
cargo publish
```

The first upload publishes **0.1.0** permanently; you cannot replace that version (only [yank](https://doc.rust-lang.org/cargo/commands/cargo-yank.html)).

## After publish

- Verify: `cargo install zerum && zerum list-checks`
- docs.rs builds automatically from the published crate (crate docs in `src/lib.rs`).

## Version bumps

1. Update `version` in `Cargo.toml`.
2. Tag in git when you tag releases: `git tag v0.2.0 && git push origin v0.2.0` on [latentmeta/zerum](https://github.com/latentmeta/zerum) (optional).
3. `cargo publish` again.
