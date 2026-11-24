# groff

GNU replacement for the `troff` and `nroff` typesetting utilities.

## Usage
- `groff path/to/input.roff >path/to/output.ps` : format output for a PostScript printer, saving the output to a file
- `groff -man -T ascii path/to/manpage.1 | less --RAW-CONTROL-CHARS` : render a man page using the ASCII output device, and display it using a pager
- `groff -man -T html path/to/manpage.1 >path/to/manpage.html` : render a man page into an HTML file
- `groff -t -p -me -T pdf path/to/input.me >path/to/output.pdf` : typeset a roff file containing tables and pictures, using the [me] macro set, to PDF, saving the output
- `eval "$(grog -T utf8 path/to/input.me)"` : run a `groff` command with preprocessor and macro options guessed by the `grog` utility

## Categories
files, documents, media

## Links
- More information: https://www.gnu.org/software/groff/manual/groff.html.node/Groff-Options.html
- tldr page: https://tldr.inbrowser.app/pages/common/groff
- Wikipedia: https://en.wikipedia.org/wiki/Groff_(software)
