# where-object

Select objects from a collection based on their property values. Note: This command can only be used through PowerShell.

## Usage
- `Get-Alias | Where-Object -Property Name -eq name` : filter aliases by its name
- `Get-Service | Where-Object {$_.Status -eq "Stopped"}` : list all services that are currently stopped. The `$_` automatic variable represents each object that is passed to the `Where-Object` cmdlet
- `Get-Module -ListAvailable | Where-Object { $_.Name -NotLike "Microsoft*" -And $_.Name -NotLike "PS*" }` : use multiple conditions

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
system, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/where-object
