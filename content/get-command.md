# get-command

List and get available commands in the current PowerShell session. This command can only be run through PowerShell.

## Usage
- `Get-Command` : list all available PowerShell commands (aliases, cmdlets, functions) in the current computer
- `Get-Command -ListImported` : list all available PowerShell commands in the current session
- `Get-Command -Type Alias|Cmdlet|Function` : list only PowerShell aliases/cmdlets/functions available in the computer
- `Get-Command -Type Application` : list only programs or commands available on PATH in the current session
- `Get-Command -Module module` : list only PowerShell commands by the module name, e.g. `Microsoft.PowerShell.Utility` for utility-related commands
- `Get-Command command` : get the command information (e.g. version number or module name) by its name

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
development, system, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/get-command
