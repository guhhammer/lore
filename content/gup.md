# gup

Update binaries installed by `go install` in parallel.

## Usage
- `gup update` : update all installed binaries
- `gup list` : list all installed binaries, their package paths, and versions
- `gup check` : check for available updates without installing them
- `gup export -f path/to/gup.json` : export the list of installed binaries to a file
- `gup import -f path/to/gup.json` : install binaries from a `gup` configuration file
- `gup remove binary_name` : remove a specific binary
- `gup update -e binary1,binary2,...` : update all binaries except for specific ones
- `gup update --latest binary1,binary2,...` : update specific binaries to the latest version

## Categories
packages, files

## Links
- More information: https://github.com/nao1215/gup#how-to-use
- tldr page: https://tldr.inbrowser.app/pages/common/gup
