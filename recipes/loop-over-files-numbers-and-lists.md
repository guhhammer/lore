# loop over files, numbers and lists

Repeat commands for every file of a kind, a range of numbers, or the items of an array.

## Steps
- `for f in *.jpg; do echo "$f"; done` : every .jpg here
- `for i in {1..10}; do echo "$i"; done` : 1 to 10
- `for ((i = 0; i < n; i++)); do echo "$i"; done` : a range in variables
- `for x in "${list[@]}"; do echo "$x"; done` : every item of an array
- `find . -name '*.log' -print0 | while IFS= read -r -d '' f; do gzip "$f"; done` : every file under here, any name
- `seq 1 2 10` : 1, 3, 5, 7, 9: numbers for other commands

## Notes
- {1..$n} does not work: braces come before variables; use the (( )) form.

## See also
- seq
- find

## Categories
shell development
