# read a file line by line

Go through a file's lines in a script, also split into fields, without losing spaces or backslashes.

## Steps
- `while IFS= read -r line; do echo "$line"; done < file.txt` : each line as it is
- `while IFS=, read -r name age; do echo "$name is $age"; done < people.csv` : each line split at the commas
- `command | while IFS= read -r line; do echo "$line"; done` : each line of a command's output
- `mapfile -t lines < file.txt` : every line into an array at once (bash 4)

## Notes
- `for line in $(cat file)` splits at every space, not at lines: use while read.
- A last line without a newline is skipped by read: `|| [ -n "$line" ]` keeps it.

## See also
- read
- mapfile

## Categories
shell text development
