# rip

Remove files or directories by sending them to the graveyard, allowing for them to be recovered.

## Usage
- `rip path/to/file_or_directory1 path/to/file_or_directory2 ...` : remove files or directories from specified locations and place them in the graveyard
- `rip -i path/to/file_or_directory1 path/to/file_or_directory2 ...` : interactively remove files or directories, with a prompt before every removal
- `rip -s` : list all files and directories in the graveyard that were originally within the current directory
- `rip -d` : permanently delete every file and directory in the graveyard
- `rip -u` : put back the files and directories which were affected by the most recent removal
- `rip -s -u` : put back every file and directory that is listed by `rip --seance`

## Categories
files

## Links
- More information: https://github.com/nivekuil/rip#-usage
- tldr page: https://tldr.inbrowser.app/pages/common/rip
