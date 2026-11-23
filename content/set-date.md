# set-date

Change the system time on the computer to a time that you specify. Note: This command can only be used through PowerShell.

## Usage
- `Set-Date -Date (Get-Date).AddDays(3)` : add three days to the system date
- `Set-Date -Adjust -0:10:0 -DisplayHint Time` : set the system clock back 10 minutes
- `$90mins = New-TimeSpan -Minutes 90; Set-Date -Adjust $90mins` : add 90 minutes to the system clock

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/set-date
