# setfile

Set file attributes on files in an HFS+ directory.

## Usage
- `setfile -d "MM/DD/YYYY HH:MM:SS" path/to/file1 path/to/file2 ...` : set creation date for specific files
- `setfile -m "MM/DD/YYYY HH:MM:SS" path/to/file1 path/to/file2 ...` : set modification date for specific files
- `setfile -P -m "MM/DD/YYYY HH:MM:SS" path/to/file1 path/to/file2 ...` : set modification date for symlink file (not to linked file itself)

## Categories
files, macos

## Links
- More information: https://ss64.com/mac/setfile.html
- tldr page: https://tldr.inbrowser.app/pages/osx/setfile
