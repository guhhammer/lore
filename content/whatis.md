# whatis

Display one-line descriptions from manual pages.

## Usage
- `whatis command` : display a description from a man page
- `whatis -l command` : don't cut the description off at the end of the line
- `whatis -w net*` : display descriptions for all commands matching a `glob`
- `whatis -r 'wish[0-9]\.[0-9]'` : search man page descriptions with a `regex`
- `whatis -L en command` : display descriptions in a specific language

## See also
- man
- whereis

## Categories
desktop, text

## Links
- More information: https://manned.org/whatis
- tldr page: https://tldr.inbrowser.app/pages/linux/whatis
