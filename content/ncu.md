# ncu

Find newer versions of package dependencies and check outdated npm packages locally or globally. `ncu` only updates dependency versions in `package.json`. To install the new versions, run `npm install` afterwards.

## Usage
- `ncu` : list outdated dependencies in the current directory
- `ncu --global` : list outdated global `npm` packages
- `ncu --upgrade` : upgrade all dependencies in the current directory
- `ncu --interactive` : interactively upgrade dependencies in the current directory
- `ncu --target minor` : list outdated dependencies up to the highest minor version
- `ncu --filter keyword|/regex/` : list outdated dependencies that match a keyword or `regex`
- `ncu --dep dev|optional|peer|prod|packageManager` : list only a specific section of outdated dependencies
- `ncu --help` : display help

## Categories
packages, text, files

## Links
- More information: https://github.com/raineorshine/npm-check-updates
- tldr page: https://tldr.inbrowser.app/pages/common/ncu
