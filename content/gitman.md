# gitman

Language-agnostic dependency manager using Git. Clones repositories listed in `gitman.yml` and pins each one to a branch, tag, or commit.

## Usage
- `gitman init` : initialize a new `gitman.yml` configuration file in the current directory
- `gitman install` : install dependencies listed in `gitman.yml`
- `gitman update` : update all dependencies to their latest versions
- `gitman update -a dependency_name` : update a specific dependency and all its nested dependencies
- `gitman list` : display the current version of each installed dependency
- `gitman lock` : lock the current dependency versions into `gitman.yml`
- `gitman show dependency_name` : display the path to a specific installed dependency
- `gitman uninstall` : uninstall all installed dependencies

## Categories
version-control, packages, files

## Links
- More information: https://gitman.readthedocs.io
- tldr page: https://tldr.inbrowser.app/pages/common/gitman
