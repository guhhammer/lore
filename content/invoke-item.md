# invoke-item

Open files in their respective default programs. Note: This command can only be used through PowerShell.

## Usage
- `Invoke-Item -Path path\to\file` : open a file in its default program
- `Invoke-Item -Path path\to\directory\*` : open all files inside a directory
- `Invoke-Item -Path path\to\directory\*.png` : open all PNGs inside a directory
- `Invoke-Item -Path path\to\directory\* -Include *keyword*` : open all files inside a directory containing a specific keyword
- `Invoke-Item -Path path\to\directory\* -Exclude *keyword*` : open all files inside a directory except those containing a specific keyword
- `Invoke-Item -Path path\to\directory\* -WhatIf` : simulate a run to determine which files will be opened inside a directory through `Invoke-Item`

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/invoke-item
