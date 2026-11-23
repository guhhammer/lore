# removepkg

Remove a specified Slackware package.

## Usage
- `sudo removepkg package_name` : remove a package
- `removepkg -warn package_name` : generate a report of a simulated removal to `stdout`
- `sudo removepkg -preserve package_name` : reconstruct the package subtree in `/tmp/preserved_packages/package_name` and remove the package
- `removepkg -copy package_name` : copy package under `/tmp/preserved_packages/package_name` without removing
- `sudo removepkg -keep package_name` : save temporary files created by `removepkg` for debugging

## Categories
packages, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/removepkg
