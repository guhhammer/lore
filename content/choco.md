# choco

The Chocolatey package manager. Some subcommands such as `install`, `upgrade`, `pin`, etc. have their own usage documentation.

## Usage
- `choco install package_name` : install a package
- `choco upgrade package_name` : upgrade a specific installed package
- `choco upgrade all -y` : upgrade all outdated packages and automatically confirm all prompts
- `choco uninstall package_name -y` : uninstall a package and automatically confirm all prompts
- `choco search query` : search for packages by name or keyword
- `choco list` : list all packages installed on the machine
- `choco outdated` : show packages that have newer versions available
- `choco install package_name -s source` : install a package from a specific source

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, documents, monitoring, windows

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/
- tldr page: https://tldr.inbrowser.app/pages/windows/choco
