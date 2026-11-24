# count lines, words and files

Count the lines of a file or of a command's output, the words, and the files in a folder.

## Steps
- `wc -l file.txt` : the lines of a file
- `wc -l *.csv` : the lines of each file, and the total
- `wc -w file.txt` : the words
- `grep -c . file.txt` : the lines that are not empty
- `ls | wc -l` : the files and folders here
- `find . -type f | wc -l` : every file under this folder

## See also
- wc
- grep
- find

## Categories
text files
