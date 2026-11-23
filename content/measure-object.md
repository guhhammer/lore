# measure-object

Calculate the numeric properties of objects, and the characters, words, and lines in string objects, such as files of text. Note: This command can only be used through PowerShell.

## Usage
- `Get-ChildItem | Measure-Object` : count the files and folders in a directory
- `"One", "Two", "Three", "Four" | Set-Content -Path "path\to\file"; Get-Content "path\to\file"; | Measure-Object -Character -Line -Word` : pipe input to Measure-Command (objects that are piped to `Measure-Command` are available to the script block that is passed to the Expression parameter)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
text, files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/measure-object
