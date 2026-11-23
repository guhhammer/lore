# scc

Count lines of code. Written in Go.

## Usage
- `scc` : print lines of code in the current directory
- `scc path/to/directory` : print lines of code in the target directory
- `scc --by-file` : display output for every file
- `scc -f tabular|wide|json|csv|cloc-yaml|html|html-table` : display output using a specific output format (defaults to `tabular`)
- `scc -i go,java,js` : only count files with specific file extensions
- `scc --exclude-dir .git,.hg` : exclude directories from being counted
- `scc -s files|name|lines|blanks|code|comments|complexity` : display output and sort by column (defaults to by files)
- `scc -h` : display help

## Categories
text, files, desktop

## Links
- More information: https://github.com/boyter/scc#usage
- tldr page: https://tldr.inbrowser.app/pages/common/scc
