# tee-object

Save command output in a file or variable and also send it down the pipeline. Note: This command can only be used through PowerShell.

## Usage
- `Get-Process | Tee-Object -FilePath path\to\file` : output processes to a file and to the console
- `Get-Process notepad | Tee-Object -Variable proc | Select-Object processname,handles` : output processes to a variable and `Select-Object`

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, processes, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/tee-object
