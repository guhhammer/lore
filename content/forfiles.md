# forfiles

Select files to execute a specified command on.

## Usage
- `forfiles` : search for files in the current directory
- `forfiles /p path\to\directory` : search for files in a specific directory
- `forfiles /c "command"` : run the specified command for each file
- `forfiles /m glob_pattern` : search for files using a specific `glob` mask
- `forfiles /s` : search for files recursively
- `forfiles /d +5` : search for files older than 5 days

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/forfiles
- Wikipedia: https://en.wikipedia.org/wiki/Forfiles
