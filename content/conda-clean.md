# conda clean

Delete temporary or unused files: index cache, lock files, unused cache packages, tarballs, and log files.

## Usage
- `conda clean -avy` : delete all temporary or unused files verbosely and say yes to all confirmations
- `conda clean -itl` : delete only index cache, tarballs, and log files
- `conda clean -c path/to/tempfiles` : delete only temporary cache files that could not be deleted earlier due to being in use
- `conda clean -p` : delete only unused packages. Might delete packages installed with softlinks
- `conda clean -f` : force delete all writable packages. More broad than the `--all` option. Will delete packages installed with softlinks
- `conda clean -h` : display help

## Categories
packages, monitoring

## Links
- More information: https://docs.conda.io/projects/conda/en/stable/commands/clean.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-clean
