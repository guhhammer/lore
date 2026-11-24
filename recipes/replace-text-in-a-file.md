# replace text in a file

Change every place a word or a phrase appears in a file, keeping a copy if you like.

## Steps
- `sed 's/old/new/g' file.txt` : show the file with every old turned into new, without changing it
- `sed -i 's/old/new/g' file.txt` : change the file itself
- `sed -i.bak 's/old/new/g' file.txt` : the same, keeping the old one as file.txt.bak
- `sed -i '0,/old/s//new/' file.txt` : only the first one in the file (GNU sed)
- `sed -i 's|/old/path|/new/path|g' file.txt` : text with slashes: another separator

## Notes
- macOS: `sed -i '' 's/old/new/g' file.txt`.
- `sed 's/…/…/' file > file` empties the file: write to a new one, or use -i.

## See also
- sed
- perl

## Categories
text files
