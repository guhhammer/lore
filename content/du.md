# du

Disk usage: estimate and summarize file and directory space usage.

## Usage
- `du -b|k|m path/to/directory` : list the sizes of a directory and any subdirectories, in the given unit (B/KiB/MiB)
- `du -h path/to/directory` : list the sizes of a directory and any subdirectories, in human-readable form (i.e. auto-selecting the appropriate unit for each size)
- `du -sh path/to/directory` : show the size of a single directory, in human-readable units
- `du -ah path/to/directory` : list the human-readable sizes of a directory and of all the files and directories within it
- `du -h -d n path/to/directory` : list the human-readable sizes of a directory and any subdirectories, up to `n` levels deep
- `du -ch *.jpg` : list the human-readable size of all `.jpg` files in current directory, and show a cumulative total at the end
- `du -ah -t 1G|1024M|1048576K .[^.]* *` : list all files and directories (including hidden ones) above a certain threshold size (useful for investigating what is actually taking up the space)

## See also
- ncdu

## Categories
disks, files

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/du
- Wikipedia: https://en.wikipedia.org/wiki/Du_(Unix)
