# robocopy

Robust File and Folder Copy. By default files will only be copied if the source and destination have different time stamps or different file sizes.

## Usage
- `robocopy path\to\source_directory path\to\destination_directory *.jpg *.bmp` : copy all `.jpg` and `.bmp` files from one directory to another
- `robocopy path\to\source_directory path\to\destination_directory /E` : copy all files and subdirectories, including empty ones
- `robocopy path\to\source_directory path\to\destination_directory /MIR /COPYALL` : mirror/Sync a directory, deleting anything not in source, and include all attributes and permissions
- `robocopy path\to\source_directory path\to\destination_directory /E /XO` : copy all files and subdirectories, excluding source files that are older than destination files
- `robocopy path\to\source_directory path\to\destination_directory /MIN:52428800 /L` : list all files 50 MB or larger instead of copying them
- `robocopy path\to\source_directory path\to\destination_directory /Z /R:5 /W:15` : allow resuming if network connection is lost and limit retries to 5 and wait time to 15 sec
- `robocopy /?` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/robocopy
