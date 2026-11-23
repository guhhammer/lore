# plutil

View, convert, validate, or edit property list ("plist") files.

## Usage
- `plutil -p file1.plist file2.plist ...` : display the contents of one or more plist files in human-readable format
- `plutil -convert xml1 file1.plist file2.plist ...` : convert one or more plist files to XML format, overwriting the original files in-place
- `plutil -convert binary1 file1.plist file2.plist ...` : convert one or more plist files to binary format, overwriting the original files in-place
- `plutil -convert xml1|binary1|json|swift|objc path/to/file.plist -o path/to/new_file.plist` : convert a plist file to a different format, writing to a new file
- `plutil -convert xml1|binary1|json|swift|objc path/to/file.plist -o -` : convert a plist file to a different format, writing to `stdout`

## Categories
files, text, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/plutil.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/plutil
