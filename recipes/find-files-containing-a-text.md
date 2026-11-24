# find files containing a text

Search every file under a folder for a word or a phrase, and show where it is.

## Steps
- `grep -rn 'some text' .` : every line containing the text, under this folder, with file and line number
- `grep -rl 'some text' .` : only the names of the files that contain it
- `grep -rni 'some text' .` : the same, ignoring upper and lower case
- `grep -rnw 'word' .` : only where it is a whole word
- `grep -rn --include='*.py' 'some text' .` : only in the files of one kind
- `grep -rn --exclude-dir={.git,node_modules} 'some text' .` : skipping folders you do not want searched
- `rg 'some text'` : the same, faster, with ripgrep, which skips what .gitignore lists

## Notes
- `-I` skips binary files, and `2>/dev/null` hides the files you may not read.

## See also
- grep
- rg
- find

## Categories
files text search
