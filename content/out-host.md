# out-host

Handle and output `stdout` of any command within PowerShell session. Can also be used as a substitute of `more`. Note: This command can only be used through PowerShell.

## Usage
- `command_or_variable | Out-Host` : display a command output or variable's value into the command-line (equivalent to invoking the command or variable without `Out-Host` itself)
- `command_or_variable | Out-Host -Paging` : display the output into an interactive paging view (will not work in PowerShell ISE)
- `Get-Content path\to\file | Out-Host -Paging` : interactively display a text file within the paging view (equivalent to `more path/to/file`)
- `<Space>` : go to the next page within the paging view
- `<Enter>` : go to the next line within the paging view
- `<q>` : exit the paging view

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, files, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/out-host
