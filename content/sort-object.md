# sort-object

Sort objects by property values. Note: This command can only be used through PowerShell.

## Usage
- `Get-ChildItem | Sort-Object` : sort the current directory by name
- `Get-ChildItem | Sort-Object -Descending` : sort the current directory by name descending
- `"a", "b", "a" | Sort-Object -Unique` : sort items removing duplicates
- `Get-ChildItem | Sort-Object -Property Length` : sort the current directory by file length
- `Get-Process | Sort-Object -Property WS` : sort processes with the highest memory usage based on their working set (WS) size

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/sort-object
