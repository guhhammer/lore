# source

Execute commands from a file in the current shell.

## Usage
- `source path/to/file` : evaluate contents of a given file
- `source path/to/file argument1 argument2 ...` : evaluate a file with arguments
- `source file` : search and evaluate a file from `$PATH`
- `source -p path/to/directory1:path/to/directory2:... file` : search and evaluate a file in a given set of directories
- `. path/to/file` : evaluate contents of a given file (alternatively replacing `source` with `.`)

## Categories
files, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-source
- tldr page: https://tldr.inbrowser.app/pages/common/source
- Wikipedia: https://en.wikipedia.org/wiki/Dot_(command)
