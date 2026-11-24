# delete lines containing a text

Remove every line of a file that contains a word, or keep only those lines.

## Steps
- `grep -v 'text' file.txt > new.txt` : write the file without those lines to a new file
- `sed -i '/text/d' file.txt` : delete them in the file itself
- `sed -i.bak '/text/d' file.txt` : the same, keeping the old file as file.txt.bak
- `grep 'text' file.txt` : the opposite: show only the lines that contain it
- `grep -c 'text' file.txt` : count how many lines contain it

## Notes
- macOS: `sed -i '' '/text/d' file.txt` (its sed needs a value after -i).

## See also
- grep
- sed

## Categories
text files
