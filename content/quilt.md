# quilt

Manage a series of patches.

## Usage
- `quilt import path/to/file.patch` : import an existing patch from a file
- `quilt new filename.patch` : create a new patch
- `quilt add path/to/file` : add a file to the current patch
- `quilt refresh` : after editing the file, refresh the current patch with the changes
- `quilt push -a` : apply all the patches in the series file
- `quilt pop -a` : remove all applied patches

## Categories
files, version-control

## Links
- More information: https://manned.org/quilt
- tldr page: https://tldr.inbrowser.app/pages/common/quilt
- Wikipedia: https://en.wikipedia.org/wiki/Quilt_(software)
