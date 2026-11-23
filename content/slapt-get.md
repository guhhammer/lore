# slapt-get

An `apt` like system for Slackware package management. Package sources need to be configured in the slapt-getrc file.

## Usage
- `slapt-get --update` : update the list of available packages and versions
- `slapt-get --install package` : install a package, or update it to the latest available version
- `slapt-get --remove package` : remove a package
- `slapt-get --upgrade` : upgrade all installed packages to their latest available versions
- `slapt-get --search query` : locate packages by the package name, disk set, or version
- `slapt-get --show package` : show information about a package

## Categories
packages, files

## Links
- More information: https://software.jaos.org/git/slapt-get/plain/README
- tldr page: https://tldr.inbrowser.app/pages/linux/slapt-get
