# tlmgr info

Show information about TeX Live packages.

## Usage
- `tlmgr info` : list all available TeX Live packages, prefixing installed ones with `i`
- `tlmgr info collections` : list all available collections
- `tlmgr info scheme` : list all available schemes
- `tlmgr info package` : show information about a specific package
- `tlmgr info package --list` : list all files contained in a specific package
- `tlmgr info --only-installed` : list all installed packages
- `tlmgr info package --data "name,category,installed,size,depends,..."` : show only specific information about a package
- `tlmgr info --json` : print all available packages as JSON encoded array

## Categories
packages

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#info
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-info
