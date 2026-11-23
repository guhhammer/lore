# pamac

Utility for the GUI package manager pamac. If you can't see the AUR packages, enable it in `/etc/pamac.conf` or in the GUI.

## Usage
- `pamac install package_name` : install a new package
- `pamac remove --orphans package_name` : remove a package and its no longer required dependencies (orphans)
- `pamac search package_name` : search the package database for a package
- `pamac list --installed` : list installed packages
- `pamac checkupdates` : check for package updates
- `pamac upgrade` : upgrade all packages

## Categories
packages

## Links
- More information: https://wiki.manjaro.org/index.php/Pamac
- tldr page: https://tldr.inbrowser.app/pages/linux/pamac
