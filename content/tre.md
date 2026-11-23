# tre

Show the contents of the current directory as a tree. Respects the `.gitignore` settings by default.

## Usage
- `tre -d` : print directories only
- `tre -j` : print JSON containing files in the tree hierarchy instead of the normal tree diagram
- `tre -l depth` : print files and directories up to the specified depth limit (where 1 means the current directory)
- `tre -a -c automatic|always|never` : print all hidden files and directories using the specified colorization mode
- `tre -e command` : print files within the tree hierarchy, assigning a shell alias to each file that, when called, will open the associated file using the provided `command` (or in `$EDITOR` by default)
- `tre -E regex` : print files within the tree hierarchy, excluding all paths that match the provided `regex`
- `tre -h` : display help
- `tre -V` : display version

## Categories
files, documents, text

## Links
- More information: https://github.com/dduan/tre#everything-else
- tldr page: https://tldr.inbrowser.app/pages/common/tre
