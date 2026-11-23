# pacman-deptest

Check each dependency specified and return a list of dependencies that are not currently satisfied on the system.

## Usage
- `pacman -T package1 package2 ...` : print the package names of the dependencies that are not installed
- `pacman -T "bash>=5"` : check if the installed package satisfies the given minimum version
- `pacman -T "bash>5"` : check if a later version of a package is installed
- `pacman -Th` : display help

## Categories
packages

## Links
- More information: https://manned.org/pacman.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-deptest
