# uv add

Add package dependencies to the `pyproject.toml` file. Packages are specified according to <https://peps.python.org/pep-0508/>.

## Usage
- `uv add package` : add the latest version of a package
- `uv add package1 package2 ...` : add multiple packages
- `uv add package>=1.2.3` : add a package with a version requirement
- `uv add --optional optional package1 package2 ...` : add packages to an optional dependency group, which will be included when published
- `uv add --group group package1 package2 ...` : add packages to a local group, which will not be included when published
- `uv add --dev package1 package2 ...` : add packages to the dev group, shorthand for `--group dev`
- `uv add --editable path/to/package/` : add package as editable
- `uv add package --extra extra_feature` : enable an extra when installing package, may be provided multiple times

## Categories
packages, users, network

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-add
- tldr page: https://tldr.inbrowser.app/pages/common/uv-add
