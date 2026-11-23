# sk

Fuzzy finder written in Rust.

## Usage
- `find path/to/directory -type f | sk` : start `skim` on all files in the specified directory
- `ps aux | sk` : start `skim` for running processes
- `sk --query "query"` : start `skim` with a specified query
- `find path/to/directory -type f | sk --multi >path/to/file` : select multiple files with `<Shift Tab>` and write to a file

## See also
- fzf

## Categories
files, databases

## Links
- More information: https://github.com/skim-rs/skim
- tldr page: https://tldr.inbrowser.app/pages/common/sk
