# uv tree

Display project dependencies in a tree format.

## Usage
- `uv tree` : show dependency tree for current environment
- `uv tree --universal` : show dependency tree for all environments
- `uv tree -d n` : show dependency tree up to a certain depth
- `uv tree --outdated` : show the latest available version for all outdated packages
- `uv tree --no-dev` : exclude dependencies from the dev group
- `uv tree --invert` : show the inverted tree, so children are dependents instead of dependencies

## Categories
packages, desktop, development

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-tree
- tldr page: https://tldr.inbrowser.app/pages/common/uv-tree
