# equery

View information about Portage packages.

## Usage
- `equery list '*'` : list all installed packages
- `equery list -po package1 package2 ...` : search for installed packages in the Portage tree and in overlays
- `equery depends package` : list all packages that depend on a given package
- `equery depgraph package` : list all packages that a given package depends on
- `equery files --tree package` : list all files installed by a package

## Categories
packages

## Links
- More information: https://wiki.gentoo.org/wiki/Equery
- tldr page: https://tldr.inbrowser.app/pages/linux/equery
