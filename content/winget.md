# winget

Windows Package Manager.

## Usage
- `winget add package` : install a package (use `--source` when package is available from multiple sources)
- `winget rm package` : remove a package (Note: `remove` can also be used instead of `uninstall`)
- `winget show package` : display information about a package
- `winget search package` : search for a package
- `winget upgrade -r` : upgrade all packages to the latest versions
- `winget ls -s winget` : list all packages installed that can be managed with `winget`
- `winget import|export --import-file|--output path\to\file` : import packages from a file, or export installed packages to a file
- `winget validate path\to\manifest` : validate manifests before submitting a PR to the winget-pkgs repository

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, files, windows

## Links
- More information: https://learn.microsoft.com/windows/package-manager/winget
- tldr page: https://tldr.inbrowser.app/pages/windows/winget
