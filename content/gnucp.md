# gnucp

This command is an alias of GNU `cp`.

## Usage
- `cp path/to/source_file path/to/target_file` : copy a file to another location
- `cp path/to/source_file path/to/target_parent_directory` : copy a file into another directory, keeping the filename
- `cp -r path/to/source_directory path/to/target_directory` : recursively copy a directory's contents to another location (if the destination exists, the directory is copied inside it)
- `cp -vra path/to/source_directory path/to/target_directory` : copy a directory recursively, in verbose mode (shows files as they are copied) and preserve permission, timestamps, and symlinks
- `cp -t path/to/destination_directory path/to/file1 path/to/file2 ...` : copy multiple files at once to a directory
- `cp -i *.ext path/to/target_directory` : copy all files with a specific extension to another location, in interactive mode (prompts user before overwriting)
- `cp -L link path/to/target_directory` : follow symbolic links before copying
- `cp --parents source/path/to/file path/to/target_directory` : use the path to the source files given from the command line, creating any missing intermediate directories when copying

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnucp
