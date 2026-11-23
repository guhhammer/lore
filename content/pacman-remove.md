# pacman-remove

Remove packages from the system.

## Usage
- `sudo pacman -Rs package` : remove a package and its dependencies recursively
- `sudo pacman -Rsn package` : remove a package and its dependencies. Also do not save backups of configuration files
- `sudo pacman -R --noconfirm package` : remove a package without prompting
- `sudo pacman -Rsn $(pacman -Qdtq)` : remove orphan packages (installed as dependencies but not required by any package)
- `sudo pacman -Rc package` : remove a package and cascade that to all packages that depend on it
- `pacman -Rp package` : list and print packages that would be affected (does not Remove any packages)
- `pacman -Rh` : display help

## Categories
packages, users

## Links
- More information: https://manned.org/pacman.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-remove
