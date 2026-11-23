# atool

A script for managing file archives of various types. `atool` uses external archiver programs but provides a consistent command-line interface for listing, extracting, creating, and managing archives.

## Usage
- `atool -l path/to/archive.zip` : list files in an archive
- `atool -x archive.tar.gz` : extract an archive (safely creates a subdirectory if needed)
- `atool -X path/to/output_directory archive.rar` : extract an archive to a specific directory
- `atool -c archive.tar path/to/file_in_archive.txt` : display a specific file's content from an archive to `stdout` (like `cat`)
- `atool -a new_archive.zip path/to/file1 path/to/file2 ...` : create a new archive from specified files and/or directories
- `atool -l -p large_archive.tar.bz2` : list files in an archive and send the output through a pager
- `atool -x -e archive1.zip archive2.tar.gz *.rar` : extract multiple archives at once (each to its own subdirectory if needed)
- `atool -r old_archive.tar.gz new_archive.tar.7z` : repack an archive from one format to another (e.g., `.tar.gz` to `.tar.7z`)

## Categories
archives, files, network

## Links
- More information: https://manned.org/atool
- tldr page: https://tldr.inbrowser.app/pages/common/atool
