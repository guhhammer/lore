# xcopy

Copy files and directory trees.

## Usage
- `xcopy path\to\file_or_directory path\to\destination_directory` : copy the file(s) to the specified destination
- `xcopy path\to\file_or_directory path\to\destination_directory /p` : list files that will be copied before copying
- `xcopy path\to\file_or_directory path\to\destination_directory /t` : copy the directory structure only, excluding files
- `xcopy path\to\file_or_directory path\to\destination_directory /e` : include empty directories when copying
- `xcopy path\to\file_or_directory path\to\destination_directory /o` : keep the source ACL in the destination
- `xcopy path\to\file_or_directory path\to\destination_directory /z` : allow resuming when network connection is lost
- `xcopy path\to\file_or_directory path\to\destination_directory /y` : disable the prompt when the file exists in the destination
- `xcopy /?` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/xcopy
