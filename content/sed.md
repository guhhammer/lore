# sed

Edit text in a scriptable manner.

## Usage
- `command | sed 's/apple/mango/g'` : replace all `apple` (basic `regex`) occurrences with `mango` (basic `regex`) in all input lines and print the result to `stdout`
- `command | sed -f path/to/script.sed` : execute a specific script file and print the result to `stdout`
- `command | sed -n '1p'` : print just a first line to `stdout`

## See also
- awk
- ed

## Categories
text

## Links
- More information: https://manned.org/sed.1posix
- tldr page: https://tldr.inbrowser.app/pages/common/sed
- Wikipedia: https://en.wikipedia.org/wiki/Sed
