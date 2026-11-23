# mamba repoquery

Efficiently query conda and mamba package repositories and package dependencies.

## Usage
- `mamba repoquery search package` : search for all available versions of a particular package
- `mamba repoquery search "sphinx<5"` : search for all packages satisfying specific constraints
- `mamba repoquery depends -t scipy` : list the dependencies of a package installed in the currently activated environment, in a tree format
- `mamba repoquery whoneeds ipython` : print packages in the current environment that require a particular package to be installed (i.e. inverse of `depends`)

## Categories
packages, databases

## Links
- More information: https://mamba.readthedocs.io/en/latest/user_guide/mamba.html#repoquery
- tldr page: https://tldr.inbrowser.app/pages/common/mamba-repoquery
