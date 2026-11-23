# czkawka_cli

Find duplicates, empty folders, similar images, and much more.

## Usage
- `czkawka_cli dup -d path/to/directory1 -d path/to/directory2 -f path/to/results.txt` : list duplicate in specific directories and write the results into a file
- `czkawka_cli dup -d path/to/directory -D AEN|AEO|ON|OO|HARD|NONE` : find duplicate files in specific directories and delete them (default: `NONE`)
- `czkawka_cli image -d path/to/directory -s Minimal|VerySmall|Small|Medium|High|VeryHigh|Original -f path/to/results.txt` : find similar looking image with a specific similarity level (default: `High`)
- `czkawka_cli -h` : display help

## Categories
files, containers, media

## Links
- More information: https://github.com/qarmin/czkawka/blob/master/czkawka_cli/README.md
- tldr page: https://tldr.inbrowser.app/pages/common/czkawka_cli
