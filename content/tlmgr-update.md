# tlmgr update

Update TeX Live packages.

## Usage
- `sudo tlmgr update --all` : update all TeX Live packages
- `sudo tlmgr update --self` : update tlmgr itself
- `sudo tlmgr update package` : update a specific package
- `sudo tlmgr update --all --exclude package` : update all except a specific package
- `sudo tlmgr update --all --backup` : update all packages, making a backup of the current packages
- `sudo tlmgr update --no-depends package` : update a specific package without updating its dependencies
- `sudo tlmgr update --all --dry-run` : simulate updating all packages without making any changes

## Categories
packages, users

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#update-option...-pkg
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-update
