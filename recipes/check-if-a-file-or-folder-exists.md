# check if a file or folder exists

Test in a script whether a path exists, is a file, a folder, empty or runnable.

## Steps
- `[ -f config.txt ] && echo 'it is a file'` : a regular file
- `[ -d logs ] || mkdir logs` : a folder; make it when it is missing
- `[ ! -e path ] && echo 'nothing there'` : nothing at all by that name
- `[ -s data.csv ] && echo 'not empty'` : a file that is not empty
- `[ -x script.sh ] && ./script.sh` : a file you may run
- `[ -L link ] && echo 'a symbolic link'` : a link

## Notes
- Quote paths that may have spaces: `[ -f "$file" ]`.
- In scripts, `if [ -d "$dir" ]; then … fi` does the same over several lines.

## See also
- test

## Categories
shell development
