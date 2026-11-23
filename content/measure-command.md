# measure-command

Measure the time it takes to run script blocks and cmdlets. Note: This command can only be used through PowerShell.

## Usage
- `Measure-Command { command }` : measure the time it takes to run a command
- `10, 20, 50 | Measure-Command -Expression { for ($i=0; $i -lt $_; $i++) {$i} }` : pipe input to Measure-Command (objects that are piped to `Measure-Command` are available to the script block that is passed to the Expression parameter)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
development, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/measure-command
