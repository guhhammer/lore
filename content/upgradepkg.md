# upgradepkg

Upgrade Slackware packages by replacing existing packages with new versions.

## Usage
- `sudo upgradepkg path/to/package.tgz` : upgrade a package
- `sudo upgradepkg --install-new path/to/package.tgz` : upgrade a package, installing it if not already present
- `sudo upgradepkg --reinstall path/to/package.tgz` : reinstall a package (even if the same version is already installed)
- `upgradepkg --dry-run path/to/package.tgz` : simulate an upgrade, showing what would happen
- `sudo upgradepkg --verbose path/to/package.tgz` : upgrade a package and show detailed progress

## See also
- installpkg
- removepkg
- makepkg
- pkgtool

## Categories
packages, files, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/upgradepkg
