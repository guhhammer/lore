# fzf

Fuzzy finder.

## Usage
- `find path/to/directory -type f | fzf` : start `fzf` on all files in the specified directory
- `ps aux | fzf` : start `fzf` for running processes
- `find path/to/directory -type f | fzf -m >path/to/file` : select multiple files with `<Shift Tab>` and write to a file
- `fzf -q "query"` : start `fzf` with a specified query
- `fzf -q "^core go$ | rb$ | py$"` : start `fzf` on entries that start with `core` and end with either `go`, `rb`, or `py`
- `fzf -q '!pyc travis'` : start `fzf` on entries that do not match `pyc` and contain `travis`

## See also
- sk

## Categories
files

## Links
- More information: https://github.com/junegunn/fzf#usage
- tldr page: https://tldr.inbrowser.app/pages/common/fzf
