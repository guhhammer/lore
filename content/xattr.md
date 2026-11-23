# xattr

Utility to work with extended filesystem attributes.

## Usage
- `xattr -l file` : list key:value extended attributes for a given file
- `xattr -w attribute_key attribute_value file` : write an attribute for a given file
- `xattr -d com.apple.quarantine file` : delete an attribute from a given file
- `xattr -c file` : delete all extended attributes from a given file
- `xattr -rd attribute_key path/to/directory` : recursively delete an attribute in a given directory

## Categories
files, disks, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/xattr.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/xattr
