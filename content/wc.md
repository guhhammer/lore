# wc

Count lines, words, and bytes.

## Usage
- `wc -l path/to/file` : count all lines in a file
- `wc -w path/to/file` : count all words in a file
- `wc -c path/to/file` : count all bytes in a file
- `wc -m path/to/file` : count all characters in a file (taking multi-byte characters into account)
- `find . | wc` : count all lines, words, and bytes from `stdin`
- `wc -L path/to/file` : count the length of the longest line in number of characters

## Categories
text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/wc
- Wikipedia: https://en.wikipedia.org/wiki/Wc_(Unix)
