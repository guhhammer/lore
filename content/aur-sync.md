# aur sync

Download and build AUR packages automatically. Note: A local repository needs to be defined in `/etc/pacman.conf` and `vifm` needs to be installed for this to fully function.

## Usage
- `aur sync package1 package2 ...` : download one or more packages and their dependencies from the AUR, build them, and add them to a local repository
- `aur sync -u` : upgrade local repository packages
- `aur sync -u --ignore package1,package2,...` : ignore specific packages when upgrading
- `aur sync -C package` : clean build files after install
- `aur sync --noview -n package` : install a package without viewing changes in Vim and do not confirm dependency installation

## Categories
packages, network, development

## Links
- More information: https://github.com/aurutils/aurutils
- tldr page: https://tldr.inbrowser.app/pages/linux/aur-sync
