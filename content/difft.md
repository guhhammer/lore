# difft

Compare files or directories based on the syntax of the programming language.

## Usage
- `difft path/to/file_or_directory1 path/to/file_or_directory2` : compare two files or directories
- `difft --check-only path/to/file1 path/to/file2` : only report the presence of differences between the files
- `difft --display side-by-side|side-by-side-show-both|inline|json path/to/file1 path/to/file2` : specify the display mode (default is `side-by-side`)
- `difft --ignore-comments path/to/file1 path/to/file2` : ignore comments when comparing
- `difft --syntax-highlight on|off path/to/file1 path/to/file2` : enable/Disable syntax highlighting of source code (default is `on`)
- `difft --skip-unchanged path/to/file_or_directory1 path/to/file_or_directory2` : do not output anything at all if there are no differences between files
- `difft --list-languages` : print all programming languages supported by the tool, along with their extensions

## See also
- delta
- diff

## Categories
files, development

## Links
- More information: https://difftastic.wilfred.me.uk/introduction.html
- tldr page: https://tldr.inbrowser.app/pages/common/difft
