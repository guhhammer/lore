# get the full path of a file

Turn a file's name into its whole path from /, through links if you want.

## Steps
- `realpath file.txt` : its absolute path, links resolved
- `readlink -f file.txt` : the same, on older systems
- `echo "$PWD/file.txt"` : this folder's path plus the name
- `find "$PWD" -name '*.txt'` : every .txt file under here, with full paths
- `pwd` : the folder you are in

## See also
- realpath
- readlink
- pwd

## Categories
files
