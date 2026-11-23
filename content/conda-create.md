# conda create

Create new conda environments.

## Usage
- `conda create -y -n py39 python=3.9 "numpy>=1.11" scipy` : create a new environment named `py39`, install Python 3.9, NumPy v1.11 or above in it, and the latest stable version of SciPy. Say yes to all confirmations
- `conda create -n myenv --file file1.yml --file file2.yml` : create a new environment named `myenv` and install packages listed in files
- `conda create -p path/to/myenv` : create a new environment at a custom path (i.e. prefix)
- `conda create --clone py39 -n py39-copy` : make exact copy of an environment named `py39`
- `conda create -h` : display help

## Categories
files, packages

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/create.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-create
