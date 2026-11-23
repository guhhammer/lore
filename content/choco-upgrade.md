# choco upgrade

Upgrade one or more packages with Chocolatey.

## Usage
- `choco upgrade package1 package2 ...` : upgrade one or more packages
- `choco upgrade package --version version` : upgrade to a specific version of a package
- `choco upgrade all` : upgrade all packages
- `choco upgrade all --except "package1,package2,..."` : upgrade all except specified comma-separated packages
- `choco upgrade package -y` : confirm all prompts automatically
- `choco upgrade package -s source_url|alias` : specify a custom source to receive packages from
- `choco upgrade package -u username -p password` : provide a username and password for authentication

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, windows

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/upgrade/
- tldr page: https://tldr.inbrowser.app/pages/windows/choco-upgrade
