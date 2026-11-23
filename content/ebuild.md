# ebuild

A low level interface to the Gentoo Portage system.

## Usage
- `ebuild path/to/file.ebuild manifest` : create or update the package manifest
- `ebuild path/to/file.ebuild clean` : clean the temporary build directories for the build file
- `ebuild path/to/file.ebuild fetch` : fetch sources if they do not exist
- `ebuild path/to/file.ebuild unpack` : extract the sources to a temporary build directory
- `ebuild path/to/file.ebuild compile` : compile the extracted sources
- `ebuild path/to/file.ebuild install` : install the package to a temporary install directory
- `ebuild path/to/file.ebuild qmerge` : install the temporary files to the live filesystem
- `ebuild path/to/file.ebuild merge` : fetch, unpack, compile, install, and qmerge the specified ebuild file

## Categories
files, packages, development

## Links
- More information: https://wiki.gentoo.org/wiki/Ebuild
- tldr page: https://tldr.inbrowser.app/pages/linux/ebuild
