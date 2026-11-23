# copy

Copy files from one location to another. In PowerShell, this command is an alias of `Copy-Item`. This documentation is based on the Command Prompt (`cmd`) version of `copy`.

## Usage
- `copy path\to\source_file path\to\target_file` : copy a file to another location
- `copy path\to\source_file path\to\target_directory` : copy a file into another directory, keeping the filename
- `copy path\to\file1 path\to\file2 ... path\to\target_directory` : copy multiple files to a directory
- `copy path\to\source_directory\* path\to\target_directory` : copy all files from one directory to another directory
- `copy path\to\source_directory\*.ext path\to\target_directory` : copy files with a specific extension
- `copy /-y path\to\source_file path\to\target_file` : copy a file and prompt before overwriting

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, shell, documents, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/copy
