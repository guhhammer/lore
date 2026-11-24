# save a command's output in a variable

Keep what a command prints in a variable, or each line in an array, to use later in a script.

## Steps
- `today=$(date +%F)` : the output of date in today
- `count=$(ls | wc -l)` : a count
- `echo "$today"` : use it; the quotes keep spaces and lines as they are
- `mapfile -t lines < <(git branch)` : each line of the output in an array (bash 4)
- `if out=$(command 2>&1); then echo ok; else echo "failed: $out"; fi` : the output and whether it worked

## Notes
- $(…) is the modern form of backticks: it nests and reads better.

## See also
- mapfile

## Categories
shell development
