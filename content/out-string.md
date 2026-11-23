# out-string

Output input objects as a string. Note: This command can only be used through PowerShell.

## Usage
- `Get-Alias | Out-String` : print host information as string
- `Get-Alias | Out-String -Stream` : convert each object to a string rather than concatenating all the objects into a single string
- `@{TestKey = ('x' * 200)} | Out-String -Width 250` : use the `Width` parameter to prevent truncation

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
text, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/out-string
