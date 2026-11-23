# conda compare

Compare packages between conda environments.

## Usage
- `conda compare file.yml` : compare packages in the current directory to packages from the file `file.yml`
- `conda compare -n myenv path/to/file.yml` : compare packages in environment named `myenv` to packages from the file `file.yml`
- `conda compare -p path/to/myenv path/to/file.yml` : compare packages in environment `myenv` at custom path (i.e. prefix) to packages from the file `file.yml`
- `conda compare -h` : display help

## Categories
files, packages

## Links
- More information: https://docs.conda.io/projects/conda/en/stable/commands/compare.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-compare
