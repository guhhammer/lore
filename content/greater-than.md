# greater-than

Redirect output.

## Usage
- `command >path/to/file` : redirect `stdout` to a file
- `command >>path/to/file` : append to a file
- `command &>path/to/file` : redirect both `stdout` and `stderr` to a file
- `command 2>/dev/null` : redirect `stderr` to `/dev/null` to keep the terminal output clean
- `>path/to/file` : clear the file contents or create a new empty file
- `command1 2>&1 | command2` : redirect `stderr` to `stdout` for piping them together
- `exec 3>path/to/file` : open a persistent file descriptor
- `echo text >&3` : write to a custom file descriptor

## Categories
files

## Links
- More information: https://gnu.org/software/bash/manual/bash.html#Redirecting-Output
- tldr page: https://tldr.inbrowser.app/pages/common/greater-than
