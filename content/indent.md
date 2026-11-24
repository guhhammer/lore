# indent

Change the appearance of a C/C++ program by inserting or deleting whitespace.

## Usage
- `indent -linux path/to/source1.c path/to/source2.c ...` : format C/C++ source according to the Linux style guide, automatically back up the original files, and replace with the indented versions
- `indent -gnu path/to/source.c -o path/to/indented_source.c` : format C/C++ source according to the GNU style, saving the indented version to a different file
- `indent -kr -il3 -nut -l120 path/to/source.c -o path/to/indented_source.c` : format C/C++ source according to the style of Kernighan & Ritchie (K&R), no tabs, 3 spaces per indent, and wrap lines at 120 characters

## Categories
files

## Links
- More information: https://www.gnu.org/software/indent/manual/indent/Option-Summary.html
- tldr page: https://tldr.inbrowser.app/pages/common/indent
- Wikipedia: https://en.wikipedia.org/wiki/Indent_(Unix)
