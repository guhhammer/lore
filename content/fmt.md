# fmt

Reformat a text file by joining its paragraphs and limiting the line width to a number of characters (75 by default).

## Usage
- `fmt path/to/file` : reformat a file
- `fmt -w n path/to/file` : reformat a file producing output lines of (at most) `n` characters
- `fmt -s path/to/file` : reformat a file without joining lines shorter than the given width together
- `fmt -u path/to/file` : reformat a file with uniform spacing (1 space between words and 2 spaces between paragraphs)

## Categories
files, text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/fmt-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/fmt
- Wikipedia: https://en.wikipedia.org/wiki/Fmt_(Unix)
