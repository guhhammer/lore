# find grep sed awk

The four classic text and file tools: find looks for files, grep for lines, sed rewrites text and awk works on columns.

## Tasks
### find: files
- `find . -name '*.log'` : files by name, from here down
- `find . -iname '*.jpg'` : the same, ignoring case
- `find . -type d -name node_modules` : only folders
- `find . -mtime -1` : changed in the last day
- `find . -size +100M` : bigger than 100 MB
- `find . -name '*.tmp' -delete` : delete matches (deletes them)
- `find . -name '*.sh' -exec chmod +x {} +` : run a command on every match
### grep: lines
- `grep 'text' FILE` : lines that contain text
- `grep -rn 'text' DIR` : search a folder, with file and line number
- `grep -i 'text' FILE` : ignore case
- `grep -v 'text' FILE` : lines that do not match
- `grep -E 'a|b' FILE` : extended patterns (or)
- `grep -c 'text' FILE` : count matching lines
- `grep -l 'text' DIR/*` : only the names of matching files
- `grep -C 3 'text' FILE` : show 3 lines of context around each match
- `grep -rn --include='*.py' 'text' .` : only some file types
### sed: rewrite
- `sed 's/old/new/g' FILE` : replace in every line, print the result
- `sed -i 's/old/new/g' FILE` : change the file itself (edits it)
- `sed -i.bak 's/old/new/g' FILE` : change it and keep a .bak copy
- `sed -n '10,20p' FILE` : print lines 10 to 20
- `sed '/pattern/d' FILE` : drop lines that match
- `sed -n '/start/,/end/p' FILE` : print a block between two patterns
### awk: columns
- `awk '{print $1}' FILE` : the first column
- `awk -F: '{print $1, $3}' /etc/passwd` : a different separator
- `awk '$3 > 100' FILE` : lines where column 3 is above 100
- `awk '{sum += $2} END {print sum}' FILE` : add up a column
- `awk 'NR > 1' FILE` : skip the first line
- `awk '{print NF}' FILE` : how many columns on each line
### Combine
- `ps aux | grep '[n]ginx'` : find a process without matching grep itself
- `sort FILE | uniq -c | sort -rn | head` : the most frequent lines

## Notes
- Quote patterns with `*` or `?` so the shell does not expand them first.
- `sed -i` on macOS needs an argument: `sed -i '' 's/a/b/' FILE`.
- Check a `find -delete` or `-exec rm` by running it without that part first.
- Try `ripgrep` (`rg`) and `fd` for faster everyday searches.

## See also
- find
- grep
- sed
- awk

## Links
- find: https://www.gnu.org/software/findutils/manual/html_mono/find.html
- grep: https://www.gnu.org/software/grep/manual/grep.html
- sed: https://www.gnu.org/software/sed/manual/sed.html
- awk: https://www.gnu.org/software/gawk/manual/gawk.html

## Categories
text files
