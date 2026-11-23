# conda remove

Remove packages from a conda environment.

## Usage
- `conda remove scipy` : remove `scipy` from the currently active environment
- `conda remove -n environment_name package1 package2 ...` : remove a list of packages from the specified environment
- `conda remove -n environment_name --all` : remove all packages and the environment itself
- `conda remove -n environment_name --all --keep-env` : remove all packages, but keep the environment

## Categories
packages

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/remove.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-remove
