# move

Move or rename files and directories. In PowerShell, this command is an alias of `Move-Item`. This documentation is based on the Command Prompt (`cmd`) version of `move`.

## Usage
- `tldr move-item` : view documentation of the equivalent PowerShell command
- `move path\to\source path\to\target` : rename a file or directory when the target is not an existing directory
- `move path\to\source path\to\existing_directory` : move a file or directory into an existing directory
- `move C:\path\to\source D:\path\to\target` : move a file or directory across drives
- `move /Y path\to\source path\to\existing_directory` : do not prompt for confirmation before overwriting existing files
- `move /-Y path\to\source path\to\existing_directory` : prompt for confirmation before overwriting existing files, regardless of file permissions

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, shell, documents, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/move
- Wikipedia: https://en.wikipedia.org/wiki/Move_(command)
