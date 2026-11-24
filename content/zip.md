# zip

Package and compress (archive) files into a Zip archive.

## Usage
- `zip -r path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : add files/directories to a specific archive
- `zip -d path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : remove files/directories from a specific archive
- `zip -r path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ... -x path/to/excluded_files_or_directories` : archive files/directories excluding specified ones
- `zip -r -0..9 path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : archive files/directories with a specific compression level (`0` - the lowest, `9` - the highest)
- `zip -re path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : create an encrypted archive with a specific password
- `zip -rs 3g path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : archive files/directories to a multi-part split Zip archive (e.g. 3 GB parts)
- `zip -sf path/to/compressed.zip` : print a specific archive contents

## See also
- unzip

## Categories
archives

## Links
- More information: https://manned.org/zip
- tldr page: https://tldr.inbrowser.app/pages/common/zip
- Wikipedia: https://en.wikipedia.org/wiki/Info-ZIP
