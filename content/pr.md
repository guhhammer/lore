# pr

Paginate or columnate files for printing.

## Usage
- `pr path/to/file1 path/to/file2 ...` : print multiple files with a default header and footer
- `pr -h "header" path/to/file1 path/to/file2 ...` : print with a custom centered header
- `pr -n -D "format" path/to/file1 path/to/file2 ...` : print with numbered lines and a custom date format
- `pr -m -T path/to/file1 path/to/file2 ...` : print all files together, one in each column, without a header or footer
- `pr +2:5 -l page_length path/to/file1 path/to/file2 ...` : print, beginning at page 2 up to page 5, with a given page length (including header and footer)
- `pr -o offset -W width path/to/file1 path/to/file2 ...` : print with an offset for each line and a truncating custom page width

## Categories
files, documents, text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/pr-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/pr
- Wikipedia: https://en.wikipedia.org/wiki/Pr_(Unix)
