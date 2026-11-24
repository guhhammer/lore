# npm access

Set access level on published packages.

## Usage
- `npm access list packages user|scope|scope:team package_name` : list packages for a user or scope
- `npm access list collaborators package_name username` : list collaborators on a package
- `npm access get status package_name` : get status of a package
- `npm access set status=public|private package_name` : set package status (public or private)
- `npm access grant read-only|read-write scope:team package_name` : grant access to a package
- `npm access revoke scope:team package_name` : revoke access to a package
- `npm access set mfa=none|publish|automation package_name` : configure two-factor authentication requirement

## Categories
packages

## Links
- More information: https://docs.npmjs.com/cli/npm-access/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-access
- Wikipedia: https://en.wikipedia.org/wiki/Npm
