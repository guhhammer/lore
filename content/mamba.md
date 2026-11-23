# mamba

Fast, cross-platform package manager, intended as a drop-in replacement for conda. Some subcommands such as `repoquery` have their own usage documentation.

## Usage
- `mamba create -n environment_name python=3.10 matplotlib` : create a new environment, installing the specified packages into it
- `mamba install -c conda-forge python=3.6 numpy` : install packages into the current environment, specifying the package channel
- `mamba update -a` : update all packages in the current environment
- `mamba repoquery search numpy` : search for a specific package across repositories
- `mamba info -e` : list all environments
- `mamba clean -pt` : remove unused packages and tarballs from the cache
- `mamba activate environment_name` : activate an environment
- `mamba list` : list all installed packages in the currently activated environment

## See also
- conda

## Categories
packages, documents, monitoring

## Links
- More information: https://mamba.readthedocs.io/en/latest/user_guide/mamba.html
- tldr page: https://tldr.inbrowser.app/pages/common/mamba
