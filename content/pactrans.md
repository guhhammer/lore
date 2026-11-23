# pactrans

Install, remove, and upgrade ALPM packages.

## Usage
- `sudo pactrans --install package_name` : install a package from a repository
- `sudo pactrans --remove package_name` : remove a package
- `sudo pactrans --sysupgrade` : upgrade all installed packages
- `sudo pactrans --file path/to/package.pkg.tar.zst` : install a package file
- `sudo pactrans local/package_to_remove repository_name/package_to_install` : replace a locally installed package with a package from a repository
- `pactrans --print-only --install package_name` : print what the transaction would do without performing it

## See also
- pacinstall
- pacremove

## Categories
packages, users, files

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/pactrans.pod
- tldr page: https://tldr.inbrowser.app/pages/linux/pactrans
