# xbps

The X Binary Package System is the package manager used by Void Linux. For equivalent commands in other package managers, see <https://wiki.archlinux.org/title/Pacman/Rosetta>. Note: `xbps` isn't a command, but a set of commands.

## Usage
- `xbps-install package` : install a new package
- `xbps-install -Su` : synchronize and update all packages
- `xbps-remove package` : remove a package
- `xbps-remove -R package` : remove a package and its dependencies
- `xbps-query -s regex|keyword --repository --regex` : search for a package in remote repositories using a `regex` or a keyword (if `--regex` is omitted)
- `xbps-query -S package` : show information about an installed package

## Categories
packages, network, text

## Links
- More information: https://docs.voidlinux.org/xbps/index.html
- tldr page: https://tldr.inbrowser.app/pages/linux/xbps
