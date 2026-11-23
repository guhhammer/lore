# tlmgr remove

Uninstall TeX Live packages. By default, removed packages will be backed up to `./tlpkg/backups` under the TL installation directory.

## Usage
- `sudo tlmgr remove package` : uninstall a TeX Live package
- `tlmgr remove --dry-run package` : simulate uninstalling a package without making any changes
- `sudo tlmgr remove --no-depends package` : uninstall a package without its dependencies
- `sudo tlmgr remove --backupdir path/to/directory package` : uninstall a package and back it up to a specific directory
- `sudo tlmgr remove --all` : uninstall all of TeX Live, asking for confirmation

## Categories
packages, files, users

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#remove-option...-pkg
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-remove
