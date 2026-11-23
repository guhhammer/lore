# move-item

Move or rename files, directories, registry keys, and other PowerShell data items. This command can only be run through PowerShell.

## Usage
- `Move-Item path\to\source path\to\target` : rename a file or directory when the target is not an existing directory
- `Move-Item path\to\source path\to\existing_directory` : move a file or directory into an existing directory
- `Move-Item -LiteralPath "path\to\source" path\to\file_or_directory` : rename or move file(s) with specific name (do not treat special characters inside strings)
- `Move-Item path\to\source1 , path\to\source2 ... path\to\existing_directory` : move multiple files into an existing directory, keeping the filenames unchanged
- `Move-Item path\to\source_key1 , path\to\source_key2 ... path\to\new_or_existing_key` : move or rename registry key(s)
- `mv -Force path\to\source path\to\target` : do not prompt for confirmation before overwriting existing files or registry keys
- `mv -Confirm path\to\source path\to\target` : prompt for confirmation before overwriting existing files, regardless of file permissions
- `mv -WhatIf path\to\source path\to\target` : simulate moving files, showing the files and directories which could be moved without executing them

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/move-item
