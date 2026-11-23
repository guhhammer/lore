# get-date

Get the current date and time. Note: This command can only be used through PowerShell.

## Usage
- `Get-Date` : display the current date and time
- `Get-Date -Format "yyyy-MM-dd HH:mm:ss"` : display the current date and time with a .NET format specifier
- `(Get-Date).ToUniversalTime()` : display the current date and time in UTC and ISO 8601 format
- `Get-Date -UnixTimeSeconds 1577836800` : convert a Unix timestamp

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/get-date
