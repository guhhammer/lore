# delete empty lines

Take out the empty lines of a file, also the ones with only spaces.

## Steps
- `sed -i '/^$/d' file.txt` : delete the empty lines
- `sed -i '/^[[:space:]]*$/d' file.txt` : also the lines with only spaces or tabs
- `grep -v '^$' file.txt > new.txt` : write the file without them to a new one
- `awk 'NF' file.txt` : show it without empty or blank lines

## Notes
- macOS: `sed -i ''` in place of `sed -i`.

## See also
- sed
- grep
- awk

## Categories
text
