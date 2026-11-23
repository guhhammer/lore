# aur

Build packages from the AUR and manage local repositories. Note: A local repository needs to be defined in `/etc/pacman.conf` and `vifm` needs to be installed for this to fully function.

## Usage
- `repo-add path/to/database.db.tar.gz` : initialize the repository that matches the path in `/etc/pacman.conf`
- `aur search keyword` : search the AUR database for a package
- `aur sync package1 package2 ...` : download one or more packages and their dependencies from the AUR, build them, and add them to a local repository
- `aur repo -l` : list packages available in your local repository
- `aur sync -u` : upgrade local repository packages
- `aur sync -C package` : clean build files after install
- `aur sync --noview -n package` : install a package without viewing changes in Vim and do not confirm dependency installation
- `repo-remove path/to/database.db.tar.gz package` : remove a package form the repository metadata (does not remove the package file itself)

## Categories
packages, development, files

## Links
- More information: https://github.com/aurutils/aurutils
- tldr page: https://tldr.inbrowser.app/pages/linux/aur
