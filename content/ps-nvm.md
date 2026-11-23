# ps-nvm

PowerShell-based utility to manage multiple Node.js versions, inspired by `nvm`. This tool provides multiple commands that all can only be run through PowerShell.

## Usage
- `Get-NodeInstallLocation` : get the current Node.js installation directory
- `Get-NodeVersions` : list all installed Node.js versions
- `Get-NodeVersions -Remote` : list all available Node.js versions
- `Install-NodeVersion node_version` : install a specific Node.js version
- `Install-NodeVersion node_version1 , node_version2 , ...` : install multiple Node.js versions
- `Remove-NodeVersion node_version` : uninstall a given Node.js version
- `Remove-NodeVersion node_version1 , node_version2 , ...` : uninstall multiple Node.js versions
- `Set-NodeInstallLocation path/to/directory` : change the Node.js install location to a specified directory (`ps-nvm` will create a new `.nvm` subdirectory to install them)

## Categories
packages, files

## Links
- More information: https://github.com/aaronpowell/ps-nvm
- tldr page: https://tldr.inbrowser.app/pages/common/ps-nvm
