# vgrep

A user friendly pager for grep.

## Usage
- `vgrep search_pattern` : recursively search the current directory for a pattern and cache it
- `vgrep` : display the contents of the cache
- `vgrep -s 4` : open the "4th" match from the cache in the default editor
- `vgrep -s c3` : display a context of "3" lines for each match in the cache
- `vgrep -s t` : display the number of matches for each directory in the tree
- `vgrep -s f` : display the number of matches for each file in the tree
- `vgrep -i` : start an interactive shell with cached matches

## See also
- ugrep
- rg

## Categories
desktop, files, users

## Links
- More information: https://github.com/vrothberg/vgrep/blob/main/docs/vgrep.1.md
- tldr page: https://tldr.inbrowser.app/pages/common/vgrep
