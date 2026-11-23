# del

Delete one or more files. In PowerShell, this command is an alias of `Remove-Item`. This documentation is based on the Command Prompt (`cmd`) version of `del`.

## Usage
- `tldr remove-item` : view the documentation of the equivalent PowerShell command
- `del file_pattern1 file_pattern2 ...` : delete one or more files or patterns
- `del file_pattern /p` : prompt for confirmation before deleting each file
- `del file_pattern /f` : force the deletion of read-only files
- `del file_pattern /s` : recursively delete file(s) from all subdirectories
- `del file_pattern /q` : do not prompt when deleting files based on a global wildcard
- `del file_pattern /a attribute` : delete files based on specified attributes
- `del /?` : display help and list available attributes

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
danger, shell, documents, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/del
