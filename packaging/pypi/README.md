# PyPI (`zerum`)

Trusted Publishing (no API token):

1. Create GitHub Environment **`pypi`**
2. On pypi.org → Publishing → pending publisher:
   - Owner: `latentmeta`
   - Repository: `zerum`
   - Workflow: `publish-pypi.yml`
   - Environment: `pypi`

```bash
pip install zerum
zerum --version
```
