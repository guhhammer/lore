# gfind

This command is an alias of GNU `find`.

## Usage
- `find path/to/directory -name '*.ext'` : find files by extension
- `find path/to/directory -path '*/path/*/*.ext' -or -name '*pattern*'` : find files matching multiple path/name patterns
- `find path/to/directory -type d -iname '*lib*'` : find directories matching a given name, in case-insensitive mode
- `find path/to/directory -name '*.py' -not -path '*/site-packages/*'` : find files matching a given pattern, excluding specific paths
- `find path/to/directory -maxdepth 1 -size +500k -size -10M` : find files matching a given size range, limiting the recursive depth to "1"
- `find path/to/directory -name '*.ext' -exec wc -l {} \;` : run a command for each file (use `{}` within the command to access the filename)
- `find path/to/directory -daystart -mtime -1 -exec tar -cvf archive.tar {} \+` : find all files modified today and pass the results to a single command as arguments
- `find path/to/directory -type f|d -empty -delete -print` : search for either empty files or directories and delete them verbosely

## Categories
files, archives, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gfind
