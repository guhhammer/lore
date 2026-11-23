# rpmconf

Handle `.rpmnew`, `.rpmsave`, and `.rpmorig` files left over by package upgrades.

## Usage
- `sudo rpmconf -a` : list leftover files and interactively choose what to do with each of them
- `sudo rpmconf -a -c` : delete orphaned `.rpmnew` and `.rpmsave` files

## See also
- rpm

## Categories
packages

## Links
- More information: https://manned.org/rpmconf.8
- tldr page: https://tldr.inbrowser.app/pages/linux/rpmconf
