# conda

Package, dependency, and environment management for any programming language. Some subcommands such as `create` have their own usage documentation.

## Usage
- `conda create -n environment_name python=3.9 matplotlib` : create a new environment, installing named packages into it
- `conda info -e` : list all environments
- `conda activate environment_name` : activate an environment
- `conda deactivate` : deactivate an environment
- `conda remove -n environment_name --all` : delete an environment (remove all packages)
- `conda install python=3.4 numpy` : install packages into the current environment
- `conda list` : list currently installed packages in current environment
- `conda clean -a` : delete unused packages and caches

## See also
- mamba

## Categories
packages, development, documents

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda
