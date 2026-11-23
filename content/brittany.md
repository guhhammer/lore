# brittany

Pretty-print Haskell source files.

## Usage
- `brittany path/to/file.hs` : format a Haskell source file and print the result to `stdout`
- `brittany --write-mode=inplace *.hs` : format all Haskell source files in the current directory in-place
- `brittany --check-mode path/to/file.hs` : check whether a Haskell source file needs changes and indicate the result through the program's exit code
- `brittany --indent 4 --columns 100 path/to/file.hs` : format a Haskell source file using the specified amount of spaces per indentation level and line length
- `brittany --config-file path/to/config.yaml path/to/file.hs` : format a Haskell source file according to the style defined in the specified configuration file

## Categories
files, documents

## Links
- More information: https://github.com/lspitzner/brittany#readme
- tldr page: https://tldr.inbrowser.app/pages/common/brittany
