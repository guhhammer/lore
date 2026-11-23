# deb-get

`apt-get` functionality for `.deb` packages published in third party repositories or via direct download. Works with Linux distributions which use `apt-get`.

## Usage
- `deb-get update` : update the list of available packages and versions
- `deb-get search package` : search for a given package
- `deb-get show package` : show information about a package
- `deb-get install package` : install a package, or update it to the latest available version
- `deb-get remove package` : remove a package (using `purge` instead also removes its configuration files)
- `deb-get upgrade` : upgrade all installed packages to their newest available versions
- `deb-get list` : list all available packages

## Categories
packages, network

## Links
- More information: https://github.com/wimpysworld/deb-get
- tldr page: https://tldr.inbrowser.app/pages/linux/deb-get
