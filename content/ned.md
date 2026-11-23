# ned

Like `grep` but with powerful replace capabilities. Unlike `sed`, as it isn't restricted to line oriented editing.

## Usage
- `ned -i -R '^[dl]og' .` : recursively search starting in the current directory, ignoring case
- `ned --colors '^[dl]og' .` : search always showing colored output
- `ned --colors never '^[dl]og' .` : search never showing colored output
- `ned -R --exclude '*.htm' '^[dl]og' .` : search ignoring certain files
- `ned 'dog' -r 'cat' .` : replace a string in all files in the current directory
- `ned 'the ([a-z]+) dog and the ([a-z]+) dog' -r 'the $2 dog and the $1 dog' .` : replace using numbered group references
- `ned '([a-z]+) dog' --case-replacements -r '\U$1\E! dog' --stdout .` : replace changing case
- `ned '^[sb]ad' -r 'happy' --stdout .` : preview results of a find and replace without updating the target files

## Categories
text

## Links
- More information: https://github.com/nevdelap/ned#ned-usage
- tldr page: https://tldr.inbrowser.app/pages/common/ned
