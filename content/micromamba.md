# micromamba

A fast, minimal, standalone package and environment manager for `conda` packages. Drop-in replacement for `conda`, ideal for CI, Docker, and lightweight setups.

## Usage
- `micromamba create -p /path/to/env python=3.11 numpy` : create a new environment at a specific path, installing named packages into it
- `micromamba activate -p /path/to/env` : activate an environment by name or path
- `micromamba run -p /path/to/env pytest tests/` : run a command inside an environment without activating it in the shell
- `micromamba install scipy pandas` : install packages into the currently active environment
- `micromamba list` : list all installed packages in the current environment
- `micromamba search package_name` : search for packages in channels or current environment
- `micromamba repoquery depends -t package_name` : query tree-like dependencies of a package
- `micromamba info` : show information about the current `micromamba` setup

## Categories
packages, files, containers

## Links
- More information: https://manned.org/micromamba
- tldr page: https://tldr.inbrowser.app/pages/common/micromamba
