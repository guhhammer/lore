# deno fmt

Format source files.

## Usage
- `deno fmt` : format all supported files in the current directory recursively
- `deno fmt path/to/file_or_directory1 path/to/file_or_directory2 ...` : format specific files or directories
- `deno fmt --check path/to/file_or_directory` : check formatting without modifying files
- `deno fmt --watch path/to/file_or_directory` : watch for changes and automatically format files
- `deno fmt --ignore=path/to/ignored_directory path/to/directory` : format files while ignoring a specific directory
- `echo "markdown_text" | deno fmt --ext md -` : format code from `stdin` and print the result to `stdout`

## Categories
files

## Links
- More information: https://docs.deno.com/runtime/reference/cli/fmt/
- tldr page: https://tldr.inbrowser.app/pages/common/deno-fmt
