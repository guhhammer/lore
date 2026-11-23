# 7za

File archiver with a high compression ratio. Similar to `7z` except that it supports fewer file types but is cross-platform.

## Usage
- `7za a path/to/archive.7z path/to/file_or_directory` : archive a file or directory
- `7za a path/to/encrypted.7z -ppassword -mhe=on path/to/archive.7z` : encrypt an existing archive (including file names)
- `7za x path/to/archive.7z` : extract an archive preserving the original directory structure
- `7za x path/to/archive.7z -opath/to/output` : extract an archive to a specific directory
- `7za x path/to/archive.7z -so` : extract an archive to `stdout`
- `7za a -t7z|bzip2|gzip|lzip|tar|... path/to/archive.7z path/to/file_or_directory` : archive using a specific archive type
- `7za l path/to/archive.7z` : list the contents of an archive
- `7za a path/to/archive.7z -mx=0|1|3|5|7|9 path/to/file_or_directory` : set the level of compression (higher means more compression, but slower)

## Categories
archives, files

## Links
- More information: https://manned.org/7za
- tldr page: https://tldr.inbrowser.app/pages/common/7za
