# rename-item

Powershell command to rename an item. Note: `ren` and `rni` can both be used as an alias for `Rename-Item`.

## Usage
- `Rename-Item -Path "path\to\file" -NewName "new_file_name"` : rename a file
- `Rename-Item -Path "path\to\directory" -NewName "new_directory_name"` : rename a directory
- `Rename-Item -Path "path\to\file" -NewName "path\to\new_file_name"` : rename and move a file
- `Rename-Item -Path "path\to\file" -NewName "new_file_name" -Force` : rename a file by force
- `Rename-Item -Path "path\to\file" -NewName "new_file_name" -Confirm` : prompt confirmation before renaming a file

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, shell, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/rename-item
