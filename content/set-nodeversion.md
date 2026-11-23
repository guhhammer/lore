# set-nodeversion

Set the default Node.js version for `ps-nvm`. Part of `ps-nvm` and can only be run under PowerShell.

## Usage
- `Set-NodeVersion node_version` : use a specific version of Node.js in the current PowerShell session
- `Set-NodeVersion ^20` : use the latest installed Node.js version 20.x
- `Set-NodeVersion node_version -Persist User` : set the default Node.js version for the current user (only applies to future PowerShell sessions)
- `Set-NodeVersion node_version -Persist Machine` : set the default Node.js version for all users (must be run as Administrator/root and only applies to future PowerShell sessions)

## Categories
users

## Links
- More information: https://github.com/aaronpowell/ps-nvm
- tldr page: https://tldr.inbrowser.app/pages/common/set-nodeversion
