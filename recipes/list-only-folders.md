# list only folders

List the folders here and not the files, or only the files.

## Steps
- `ls -d */` : the folders here
- `find . -maxdepth 1 -type d` : the same, hidden ones too
- `find . -maxdepth 1 -type f` : only the files
- `tree -d -L 2` : the folders two levels down, as a tree

## See also
- ls
- find
- tree

## Categories
files
