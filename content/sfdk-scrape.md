# sfdk scrape

Convert source code modifications to patches.

## Usage
- `sfdk scrape` : save source modifications as patches
- `sfdk scrape -n` : simulate saving modifications as patches, listing the commits that would be converted
- `sfdk scrape --stable` : scrape while preserving the original patches file names
- `sfdk scrape -o path/to/output_directory` : scrape while saving patches to a specified output directory
- `sfdk scrape --keep` : scrape without removing commits from submodules after creating patches

## Categories
development, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/sfdk-scrape
