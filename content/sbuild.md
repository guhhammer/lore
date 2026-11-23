# sbuild

Build a Debian binary package in a clean `chroot` environment.

## Usage
- `sbuild` : build the package in the current directory
- `sbuild package` : build the given package
- `sbuild --dist distribution` : build for a certain distribution
- `sbuild --extra-package path/to/file_or_directory` : build using custom dependencies (if a directory is passed, all files ending with `.deb` are used)
- `sbuild --build-failed-commands=%SBUILD_SHELL` : run a shell in case of build failure to further investigate
- `sbuild --host architecture` : cross build for a certain architecture
- `sbuild --arch architecture` : build for the given native architecture

## Categories
development, packages, files

## Links
- More information: https://wiki.debian.org/sbuild
- tldr page: https://tldr.inbrowser.app/pages/common/sbuild
