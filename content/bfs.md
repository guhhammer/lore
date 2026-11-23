# bfs

Breadth-first search for your files.

## Usage
- `bfs path/to/directory -name '*.ext'` : find files by extension
- `bfs path/to/directory -path '**/path/**/*.ext' -or -name '*pattern*'` : find files matching multiple path/name patterns
- `bfs path/to/directory -type d -iname '*lib*'` : find directories matching a given name, in case-insensitive mode
- `bfs path/to/directory -name '*.py' -not -path '*/site-packages/*'` : find files matching a given pattern, excluding specific paths
- `bfs path/to/directory -maxdepth 1 -size +500k -size -10M` : find files matching a given size range, limiting the recursive depth to "1"
- `bfs path/to/directory -name '*.ext' -exec wc -l {} \;` : run a command for each file (use `{}` within the command to access the filename)
- `bfs path/to/directory -daystart -mtime -1 -exec tar -cvf archive.tar {} \+` : find all files modified today and pass the results to a single command as arguments
- `bfs path/to/directory -type f|d -empty -delete -print` : find empty files (0 byte) or directories and delete them verbosely

## Categories
files, archives

## Links
- More information: https://manned.org/bfs
- tldr page: https://tldr.inbrowser.app/pages/common/bfs
