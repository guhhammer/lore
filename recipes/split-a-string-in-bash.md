# split a string in bash

Cut a string at a delimiter into parts or an array.

## Steps
- `IFS=, read -r a b c <<< "x,y,z"` : three parts into three variables
- `IFS=, read -ra parts <<< "$line"` : every part into an array
- `echo "${parts[1]}"` : the second part (arrays start at 0)
- `cut -d, -f2 <<< "$line"` : only the second field, with cut
- `tr ',' '\n' <<< "$line"` : one part a line

## See also
- read
- cut
- tr

## Categories
shell development
