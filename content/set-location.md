# set-location

Display the current working directory or move to a different directory. Note: This command can only be used through PowerShell.

## Usage
- `Set-Location path\to\directory` : go to the specified directory
- `Set-Location C:path\to\directory` : go to a specific directory in a different drive
- `Set-Location path\to\directory -PassThru` : go and display the location of specified directory
- `Set-Location ..` : go up to the parent of the current directory
- `Set-Location ~` : go to the home directory of the current user
- `Set-Location -|+` : go back/forward to the previously chosen directory
- `Set-Location \` : go to root of current drive

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/set-location
