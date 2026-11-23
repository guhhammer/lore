# remove-nodeversion

Uninstall Node.js runtime versions for `ps-nvm`. This command is part of `ps-nvm` and can only be run under PowerShell.

## Usage
- `Remove-NodeVersion node_version` : uninstall a given Node.js version
- `Remove-NodeVersion node_version1 , node_version2 , ...` : uninstall multiple Node.js versions
- `Get-NodeVersions -Filter ">=20.0.0 <21.0.0" | Remove-NodeVersion` : uninstall all currently-installed versions of Node.js 20.x
- `Get-NodeVersions | Remove-NodeVersion` : uninstall all currently-installed versions of Node.js

## Links
- More information: https://github.com/aaronpowell/ps-nvm
- tldr page: https://tldr.inbrowser.app/pages/common/remove-nodeversion
