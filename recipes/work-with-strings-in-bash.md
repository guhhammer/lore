# work with strings in bash

Join, cut, replace, change the case of and trim strings in a script, without other programs.

## Steps
- `full="$first $last"` : join strings
- `text+=' more'` : add to the end
- `echo "${text:0:5}"` : the first 5 characters
- `echo "${text//old/new}"` : replace every old with new
- `echo "${text,,} ${text^^}"` : lower case and upper case (bash 4)
- `echo "${#text}"` : its length
- `trimmed=$(echo "$text" | xargs)` : without spaces at the ends

## See also
- xargs
- printf

## Categories
shell development
