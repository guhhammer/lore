# xdelta

A delta encoding utility to generate and apply patches to binary files. For the modern version (v3), see `xdelta3`.

## Usage
- `xdelta delta path/to/old_file path/to/new_file path/to/patch_file` : generate a patch (delta) between two files
- `xdelta patch path/to/patch_file path/to/old_file path/to/new_file` : apply a patch to an old file to reconstruct the new file
- `xdelta delta -0..9 path/to/old_file path/to/new_file path/to/patch_file` : generate a patch with a specific compression level

## Categories
files, version-control

## Links
- More information: https://manned.org/xdelta
- tldr page: https://tldr.inbrowser.app/pages/common/xdelta
