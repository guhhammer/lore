# choco pin

Pin a package at a version with Chocolatey. Pinned packages are skipped automatically when upgrading.

## Usage
- `choco pin list` : display a list of pinned packages and their versions
- `choco pin add -n package` : pin a package at its current version
- `choco pin add -n package --version version` : pin a package at a specific version
- `choco pin remove -n package` : remove a pin for a specific package

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, windows

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/pin/
- tldr page: https://tldr.inbrowser.app/pages/windows/choco-pin
