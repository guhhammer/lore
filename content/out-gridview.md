# out-gridview

Display command outputs and objects into an interactive GUI window, allowing scrolling and search. Supports parsing columns from PowerShell objects, and interactive filtering of `stdout` to be passed through `stdin`. Note: This command can only be used through PowerShell and only within Windows.

## Usage
- `command | Out-GridView` : display the command or cmdlet outputs into a new window
- `$PSVersionTable | Out-GridView -Wait` : display a PowerShell hashtable (such as `$PSVersionTable`), and do not finish the command until the window has been closed
- `Import-Csv path\to\file.csv | Out-GridView -Title "title"` : read and parse a CSV file into a new window, with a custom title
- `previous_command | Out-GridView -PassThru | next_command` : interactively select a subset of rows from a previous command's `stdout` to the `stdin` input for another command (use `<Ctrl LeftClick>`, `<Shift LeftClick>`, `<Shift ArrowUp>`, or `<Shift ArrowDown>` for multi-selection)
- `Get-Content path\to\file.txt | Out-GridView -PassThru | command` : interactively select a subset of rows of a text file to be passed as an input for another command

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
desktop, files, text, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/out-gridview
