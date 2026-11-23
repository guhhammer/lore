# rar

The RAR archiver. Supports multi-volume archives that can be optionally self-extracting.

## Usage
- `rar a path/to/archive_name.rar path/to/file1 path/to/file2 path/to/file3 ...` : archive 1 or more files
- `rar a path/to/archive_name.rar path/to/directory` : archive a directory
- `rar a -v50M -R path/to/archive_name.rar path/to/file_or_directory` : split the archive into parts of equal size (50M)
- `rar a -ppassword path/to/archive_name.rar path/to/file_or_directory` : password protect the resulting archive
- `rar a -hppassword path/to/archive_name.rar path/to/file_or_directory` : encrypt file data and headers with password
- `rar a -m0..5 path/to/archive_name.rar path/to/file_or_directory` : use a specific compression level (0-5)

## Categories
archives, files, disks

## Links
- More information: https://manned.org/rar
- tldr page: https://tldr.inbrowser.app/pages/common/rar
