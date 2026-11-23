# set-clipboard

PowerShell command to set content to clipboard. Note: `scb` can be used as an alias for `Set-Clipboard`.

## Usage
- `Set-Clipboard -Value "text"` : copy text to the clipboard
- `Set-Clipboard -Value @("text 1", "text 2", "text 3")` : copy multiple texts to clipboard separated by new line
- `Set-Clipboard -Path "path\to\files_or_directories"` : copy files or directories to clipboard
- `Set-Clipboard -Path "path\to\file1","path\to\file2","path\to\file3"` : copy multiple files
- `Set-Clipboard ""` : clear the clipboard

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
desktop, files, text, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/set-clipboard
