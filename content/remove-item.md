# remove-item

Delete files, folders, as well as registry keys and subkeys. This command can only be run through PowerShell.

## Usage
- `Remove-Item path\to\file_or_key1 , path\to\file_or_key2 ...` : remove specific files or registry keys (without subkeys)
- `Remove-Item -Force path\to\file1 , path\to\file2 ...` : remove hidden or read-only files
- `Remove-Item -Confirm path\to\file_or_key1 , path\to\file_or_key2 ...` : remove specific files or registry keys interactively prompting before each removal
- `Remove-Item -Recurse path\to\file_or_directory1 , path\to\file_or_directory2 ...` : remove specific files and directories recursively (Windows 10 version 1909 or later)
- `Remove-Item -Recurse path\to\key1 , path\to\key2 ...` : remove specific Windows registry keys and all its subkeys
- `Remove-Item -WhatIf path\to\file1 , path\to\file2 ...` : simulate the deletion process

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/remove-item
