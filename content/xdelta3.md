# xdelta3

A delta compression tool for binary files.

## Usage
- `xdelta3 -e -s path/to/old_file path/to/new_file path/to/patch_file` : create a patch (encode) based on a source file
- `xdelta3 -d -s path/to/old_file path/to/patch_file path/to/new_file` : apply a patch (decompress) to a source file
- `xdelta3 -0..9 -s path/to/old_file path/to/new_file path/to/patch_file` : create a patch with a specific compression level

## Categories
files, version-control

## Links
- More information: https://github.com/jmacd/xdelta/blob/wiki/CommandLineSyntax.md
- tldr page: https://tldr.inbrowser.app/pages/common/xdelta3
