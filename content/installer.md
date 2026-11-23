# installer

Install system software and packages to a specified domain or volume.

## Usage
- `sudo installer -verbose -pkg path/to/python-version.pkg -tgt /` : install a Python package to the root volume with verbose output
- `installer -pkginfo -pkg path/to/package.pkg` : display list of packages (or subpackages for `.mpkg`) that can be installed on the target volume
- `installer -volinfo -pkg path/to/package.pkg` : display list of volumes onto which the package can be installed
- `installer -dominfo -pkg path/to/package.pkg` : display list of domains into which the package can be installed
- `installer -pkg path/to/package.pkg -showChoiceChangesXML` : generate XML of install choices for a package
- `sudo installer -pkg path/to/package.pkg -file path/to/config-file` : install package using XML config instead of command line parameters

## Categories
packages, files, disks, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/installer.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/installer
