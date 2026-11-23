# apt-mark

Utility to change the status of installed packages.

## Usage
- `sudo apt-mark auto package` : mark a package as automatically installed
- `sudo apt-mark hold package` : hold a package at its current version and prevent updates to it
- `sudo apt-mark unhold package` : allow a package to be updated again
- `apt-mark showmanual` : show manually installed packages
- `apt-mark showhold` : show held packages that aren't being updated

## Categories
packages, users

## Links
- More information: https://manned.org/apt-mark
- tldr page: https://tldr.inbrowser.app/pages/linux/apt-mark
