# grep

Searches text for lines that match a pattern, in files or in another
command's output, and prints those lines.

## Usage
- `grep error app.log` : the lines of app.log that contain "error"
- `grep -i error app.log` : the same, ignoring upper and lower case
- `grep -r TODO src/` : search every file under a folder
- `grep -n main file.c` : show line numbers
- `grep -v '^#' config` : the lines that do NOT match (here: not comments)
- `ps aux | grep firefox` : filter another command's output

## See also
- rg
- regex

## Categories
text

## Links
- GNU manual: https://www.gnu.org/software/grep/manual/grep.html
- Wikipedia: https://en.wikipedia.org/wiki/Grep
