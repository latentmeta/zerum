# PyPI (`zerum`)

End-user install and usage: see the root [README](../../README.md).

## Maintainer: Trusted Publishing

No API token. One-time setup:

1. Create GitHub Environment **`pypi`**
2. On [pypi.org](https://pypi.org) → Publishing → pending publisher:
   - Owner: `latentmeta`
   - Repository: `zerum`
   - Workflow: `publish-pypi.yml`
   - Environment: `pypi`

Tag `v0.4.1` (or dispatch **Publish PyPI wheels**) to upload wheels/sdist.
See [docs/RELEASE_v0.4.1.md](../../docs/RELEASE_v0.4.1.md) and [DISTRO_CLOSURE_v0.4.1.md](../../docs/DISTRO_CLOSURE_v0.4.1.md).

```bash
pip install "zerum==0.4.1"
zerum --version
zerum check .
```
