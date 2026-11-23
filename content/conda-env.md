# conda env

Manage conda environments.

## Usage
- `conda env create -f path/to/file` : create an environment from an environment file (YAML, TXT, etc.)
- `conda env remove -n environment_name` : delete an environment and everything in it
- `conda env update -f path/to/file --prune` : update an environment based on an environment file
- `conda env list` : list all environments
- `conda env export` : view environment details
- `conda env config vars list` : list environment variables
- `conda env config vars set my_var=value` : set environment variables

## Categories
files

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/env/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-env
