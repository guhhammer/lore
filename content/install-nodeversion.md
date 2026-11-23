# install-nodeversion

Install Node.js runtime versions for `ps-nvm`. This command is part of `ps-nvm` and can only be run under PowerShell.

## Usage
- `Install-NodeVersion node_version` : install a specific Node.js version
- `Install-NodeVersion node_version1 , node_version2 , ...` : install multiple Node.js versions
- `Install-NodeVersion ^20` : install latest available version of Node.js 20
- `Install-NodeVersion node_version -Architecture x86|x64|arm64` : install the x86 (x86 32-bit) / x64 (x86 64-bit) / arm64 (ARM 64-bit) version of Node.js
- `Install-NodeVersion node_version -Proxy http://example.com` : use an HTTP proxy to download Node.js

## Categories
packages, network

## Links
- More information: https://github.com/aaronpowell/ps-nvm
- tldr page: https://tldr.inbrowser.app/pages/common/install-nodeversion
