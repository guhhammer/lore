# egrep

Find patterns in files using extended `regex`es. Note: This command is an alias of `grep --extended-regexp`.

## Usage
- `egrep 'a+' path/to/file` : search for one or more repeated characters
- `egrep 'a?' path/to/file` : search for zero or one occurrences of a character (optional match)
- `egrep 'a{10}' path/to/file` : search for 10 repetitions of a character
- `egrep 'a{3,7}' path/to/file` : search for 3 to 7 repetitions of a character
- `egrep 'cat|dog|mouse' path/to/file` : search for one of the listed options
- `egrep 'c(a|o|u)p' path/to/file` : search for one of the listed options inside a larger pattern
- `egrep '(aeiou)+' path/to/file` : search for a group of characters repeating one or more times
- `egrep [[:alnum:|:alpha:|:space:|...]] path/to/file` : search using standard character classes (more info: <https://www.regular-expressions.info/posixbrackets.html>)

## See also
- regex

## Categories
files, text, network

## Links
- More information: https://manned.org/egrep
- tldr page: https://tldr.inbrowser.app/pages/common/egrep
