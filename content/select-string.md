# select-string

Find text in strings and files in PowerShell. You can use `Select-String` similar to `grep` in UNIX or `findstr.exe` in Windows. Note: This command can only be used through PowerShell.

## Usage
- `Select-String -Path "path\to\file" -Pattern 'search_pattern'` : search for a pattern within a file
- `Select-String -SimpleMatch "exact_string" path\to\file` : search for an exact string (disables `regex`)
- `Select-String -Path "*.ext" -Pattern 'search_pattern'` : search for pattern in all `.ext` files in current dir
- `Select-String --Context 2,3 "search_pattern" path\to\file` : capture the specified number of lines before and after the line that matches the pattern
- `Get-Content path\to\file | Select-String --NotMatch "search_pattern"` : search `stdin` for lines that do not match a pattern

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
text, files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/select-string
