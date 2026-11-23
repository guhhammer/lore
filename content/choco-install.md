# choco install

Install one or more packages with Chocolatey.

## Usage
- `choco install package1 package2 ...` : install one or more packages
- `choco install path\to\packages_file.config` : install packages from a custom configuration file
- `choco install path\to\file` : install a specific `.nuspec` or `.nupkg` file
- `choco install package --version version` : install a specific version of a package
- `choco install package -f` : force reinstallation of a package
- `choco install package -y` : confirm all prompts automatically
- `choco install package -s source_url|alias` : specify a custom source to receive packages from
- `choco install package -u username -p password` : provide a username and password for authentication

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, files, windows

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
- tldr page: https://tldr.inbrowser.app/pages/windows/choco-install
