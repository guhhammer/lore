# create an empty file

Make a new empty file, empty an existing one, or make a file with a few lines in it.

## Steps
- `touch file.txt` : make it if it is not there; else just update its date
- `> file.txt` : make it, or empty it if it is there
- `truncate -s 0 file.txt` : empty a file, even one a program has open
- `echo 'one line' > file.txt` : make it with one line in it
- `printf 'line 1\nline 2\n' > file.txt` : with several lines

## See also
- touch
- truncate
- printf

## Categories
files
