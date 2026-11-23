# gtouch

This command is an alias of GNU `touch`.

## Usage
- `touch path/to/file1 path/to/file2 ...` : create specific files or update their timestamps if they already exist
- `touch -c -a|-m path/to/file1 path/to/file2 ...` : set the file access or modification times to the current one and don't create file if it doesn't exist
- `touch -c -t YYYYMMDDHHMM.SS path/to/file1 path/to/file2 ...` : set the file time to a specific value and don't create file if it doesn't exist
- `touch -c -r path/to/reference_file path/to/file1 path/to/file2 ...` : set the files' timestamp to the reference file's timestamp, and do not create the file if it does not exist
- `touch -d "last year|5 hours|next thursday|nov 14|..." path/to/file` : set the timestamp by parsing a string
- `touch path/to/file{1..10}` : create multiple files with an increasing number
- `touch path/to/file{a..z}` : create multiple files with a letter range

## Categories
files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gtouch
