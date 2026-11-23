# vi

This command is an alias of `vim`.

## Usage
- `vim path/to/file` : open a file
- `vim +line_number path/to/file` : open a file at a specified line number
- `<:>help<Enter>` : view Vim's help manual
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : save and quit the current buffer
- `<Esc><u>` : enter normal mode and undo the last operation
- `</>search_pattern<Enter>` : search for a pattern in the file (press `<n>`/`<N>` to go to next/previous match)
- `<:>%s/regex/replacement/g<Enter>` : perform a `regex` substitution in the whole file
- `<:>set nu<Enter>` : display the line numbers

## Categories
files, editors, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/vi
