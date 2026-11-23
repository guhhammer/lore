# dpkg

Debian package manager. For equivalent commands in other package managers, see <https://wiki.archlinux.org/title/Pacman/Rosetta>. Some subcommands such as `deb` have their own usage documentation.

## Usage
- `sudo dpkg -i path/to/file.deb` : install a package
- `sudo dpkg -r package` : remove a package
- `dpkg -l pattern` : list installed packages
- `dpkg -L package` : list a package's contents
- `dpkg -c path/to/file.deb` : list contents of a local package file
- `dpkg -S path/to/file` : find out which package owns a file
- `sudo dpkg -P package` : purge an installed or already removed package, including configuration

## Categories
packages

## Links
- More information: https://manned.org/dpkg
- tldr page: https://tldr.inbrowser.app/pages/linux/dpkg
