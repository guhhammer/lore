# makepkg

Create a package which can be used with `pacman`. Uses the `PKGBUILD` file in the current working directory by default.

## Usage
- `makepkg` : make a package
- `makepkg -s` : make a package and install its dependencies
- `makepkg -si` : make a package, install its dependencies then install it to the system
- `makepkg --skipchecksums --skippgpcheck --nocheck` : make a package, but skip checking the source's hashes, PGP signatures, and the `PKGBUILD` `check()` function
- `makepkg -c` : clean up work directories after a successful build
- `makepkg --verifysource` : verify the hashes of the sources
- `makepkg --printsrcinfo >.SRCINFO` : generate and save the source information into `.SRCINFO`
- `makepkg -so` : download the source and install only the build dependencies for a program

## Categories
packages, files, network

## Links
- More information: https://manned.org/makepkg
- tldr page: https://tldr.inbrowser.app/pages/linux/makepkg
