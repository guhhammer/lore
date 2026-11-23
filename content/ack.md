# ack

A search tool like `grep`, optimized for developers.

## Usage
- `ack "search_pattern"` : search for files containing a string or `regex` in the current directory recursively
- `ack -i "search_pattern"` : search for a case-insensitive pattern
- `ack -o "search_pattern"` : search for lines matching a pattern, printing only the matched text and not the rest of the line
- `ack -t ruby "search_pattern"` : limit search to files of a specific type
- `ack -t noruby "search_pattern"` : do not search in files of a specific type
- `ack -c -h "search_pattern"` : count the total number of matches found
- `ack -c -l "search_pattern"` : print the file names and the number of matches for each file only
- `ack --help-types` : list all the values that can be used with `--type`

## See also
- rg

## Categories
text, files

## Links
- More information: https://beyondgrep.com/documentation/
- tldr page: https://tldr.inbrowser.app/pages/common/ack
