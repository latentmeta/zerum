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

Tag `v0.5.0` (or dispatch **Publish PyPI wheels**) to upload wheels/sdist.
See [docs/RELEASE_v0.5.0.md](../../docs/RELEASE_v0.5.0.md).

```bash
pip install "zerum==0.5.0"
zerum --version
zerum check . --remediation-prompt
```
