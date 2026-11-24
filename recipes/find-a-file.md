# find a file

Find a file by its name, by when it changed, or by what is written in it.

## Steps
- `find . -iname '*report*'` : files under this folder with "report" in the name, any case
- `locate -i report` : the same over the whole machine, from an index, at once
- `find . -mtime -1` : files changed in the last day
- `grep -rIl 'some text' .` : files under this folder that contain the text
- `grep -rn 'some text' .` : the lines that contain it, with file and line number
- `rg 'some text'` : the same, faster, with ripgrep

## Notes
- `locate` knows only what was there when its index was last made: `sudo updatedb` makes it again.
- In grep, `-I` skips binary files and `-l` prints only the file names.

## See also
- find
- locate
- grep
- rg
- fd

## Categories
files search
