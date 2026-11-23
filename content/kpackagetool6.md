# kpackagetool6

KPackage Manager: install, list, remove Plasma packages.

## Usage
- `kpackagetool6 --list-types` : list all known package types that can be installed
- `kpackagetool6 -t package_type -i path/to/directory` : install the package from a directory
- `kpackagetool6 -t package_type -u path/to/directory` : update installed package from a directory
- `kpackagetool6 -t Plasma/Applet -l -g` : list installed plasmoids (`--global` for all users)
- `kpackagetool6 -t Plasma/Applet -r "name"` : remove a plasmoid by name

## Categories
packages, files

## Links
- More information: https://manned.org/kpackagetool6
- tldr page: https://tldr.inbrowser.app/pages/linux/kpackagetool6
