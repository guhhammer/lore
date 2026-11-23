# installpkg

Install a Slackware package.

## Usage
- `sudo installpkg path/to/package.tgz` : install a package
- `installpkg -warn path/to/package.tgz` : generate a report of a simulated installation to `stdout`
- `installpkg -m package_name.tgz` : create a package from current directory and its subdirectories
- `sudo installpkg -r package_name.tgz` : install the contents of the current directory and subdirectories as a package with a specified name

## Categories
packages, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/installpkg
