# synopkg

Package management utility for Synology DiskStation Manager.

## Usage
- `synopkg list --name` : list the names of installed packages
- `synopkg list --depend-on package` : list packages which depend on a specific package
- `sudo synopkg start|stop package` : start/Stop a package
- `synopkg status package` : print the status of a package
- `sudo synopkg uninstall package` : uninstall a package
- `synopkg checkupdate package` : check if updates are available for a package
- `sudo synopkg upgradeall` : upgrade all packages to the latest version
- `sudo synopkg install path/to/package.spk` : install a package from a synopkg file

## Categories
packages, users

## Links
- More information: https://www.synology.com/dsm
- tldr page: https://tldr.inbrowser.app/pages/linux/synopkg
