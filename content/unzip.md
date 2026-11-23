# unzip

Extract files/directories from Zip archives.

## Usage
- `unzip path/to/archive.zip` : extract all files/directories from a specific archive into the current directory
- `unzip path/to/archive.zip -d path/to/output` : extract files/directories from an archive to a specific directory
- `unzip -c path/to/archive.zip` : extract files/directories from an archive to `stdout` alongside the extracted file names
- `unzip -O gbk path/to/archive.zip` : extract an archive created on Windows, containing files with non-ASCII (e.g. Chinese or Japanese characters) filenames
- `unzip -l path/to/archive.zip` : list the contents of a specific archive without extracting them
- `unzip -j path/to/archive.zip path/to/file1_in_archive path/to/file2_in_archive ...` : extract specific files from an archive into the current directory, discarding any directory paths

## See also
- zip

## Categories
archives

## Links
- More information: https://manned.org/unzip
- tldr page: https://tldr.inbrowser.app/pages/common/unzip
