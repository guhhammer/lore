# lpr

Print files.

## Usage
- `lpr path/to/file` : print a file to the default printer
- `lpr -# 2 path/to/file` : print 2 copies
- `lpr -P printer path/to/file` : print to a named printer
- `lpr -o page-ranges=2|2-16 path/to/file` : print either a single page (e.g. 2) or a range of pages (e.g. 2-16)
- `lpr -o sides=two-sided-long-edge|two-sided-short-edge path/to/file` : print double-sided either in portrait (long) or in landscape (short)
- `lpr -o media=a4|letter|legal path/to/file` : set page size (more options may be available depending on setup)
- `lpr -o number-up=2|4|6|9|16 path/to/file` : print multiple pages per sheet

## See also
- lpstat
- lpadmin

## Categories
files, documents, hardware

## Links
- More information: https://openprinting.github.io/cups/doc/man-lpr.html
- tldr page: https://tldr.inbrowser.app/pages/common/lpr
