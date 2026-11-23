# apk

Alpine Linux package management tool.

## Usage
- `apk upgrade -U` : update repository indexes and upgrade all packages
- `apk update` : only update repository indexes
- `apk add package` : install a new package
- `apk del package` : remove a package
- `apk fix package` : repair/Reinstall a package without modifying main dependencies
- `apk search -v keyword` : search for packages with a keyword in their name and list results with descriptions
- `apk search -d keyword` : search for packages with a keyword in their description
- `apk info package` : display information about a specific package

## Categories
packages

## Links
- More information: https://wiki.alpinelinux.org/wiki/Alpine_Package_Keeper
- tldr page: https://tldr.inbrowser.app/pages/linux/apk
