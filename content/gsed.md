# gsed

This command is an alias of GNU `sed`.

## Usage
- `command | sed 's/apple/mango/g'` : replace all `apple` (basic `regex`) occurrences with `mango` (basic `regex`) in all input lines and print the result to `stdout`
- `command | sed -f path/to/script.sed` : execute a specific script file and print the result to `stdout`
- `command | sed -n '1p'` : print just a first line to `stdout`

## Categories
text, documents, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gsed
