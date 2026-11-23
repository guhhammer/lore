# get-dedupproperties

Get Data Deduplication information. Note: This command can only be used through PowerShell.

## Usage
- `Get-DedupProperties -DriveLetter 'C'` : get Data Deduplication information of the drive
- `Get-DedupProperties -FileSystemLabel 'Label'` : get Data Deduplication information of the drive using the drive label
- `Get-DedupProperties -InputObject $(Get-Volume -DriveLetter 'E')` : get Data Deduplication information of the drive using the input object

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
disks, windows

## Links
- More information: https://learn.microsoft.com/powershell/module/storage/get-dedupproperties
- tldr page: https://tldr.inbrowser.app/pages/windows/get-dedupproperties
