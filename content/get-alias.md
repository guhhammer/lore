# get-alias

List and get command aliases in the current PowerShell session. This command can only be run under PowerShell.

## Usage
- `Get-Alias` : list all aliases in the current session
- `Get-Alias command_alias` : get the aliased command name
- `Get-Alias -Definition command` : list all aliases assigned to a specific command
- `Get-Alias abc* -Exclude *def` : list aliases that begins with `abc`, excluding those which ends at `def`

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
shell, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/get-alias
