# vim

Vim (Vi IMproved), a command-line text editor, provides several modes for different kinds of text manipulation. Pressing `<i>` in normal mode enters insert mode. Pressing `<Esc>` goes back to normal mode, which enables the use of Vim commands.

## Usage
- `vim path/to/file` : open a file
- `vim +line_number path/to/file` : open a file at a specified line number
- `<:>help<Enter>` : view Vim's help manual
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : save and quit the current buffer
- `<Esc><u>` : enter normal mode and undo the last operation
- `</>search_pattern<Enter>` : search for a pattern in the file (press `<n>`/`<N>` to go to next/previous match)
- `<:>%s/regex/replacement/g<Enter>` : perform a `regex` substitution in the whole file
- `<:>set nu<Enter>` : display the line numbers

## See also
- vimdiff
- vimtutor
- nvim
- gvim

## Categories
editors

## Links
- More information: https://www.vim.org/
- tldr page: https://tldr.inbrowser.app/pages/common/vim
- Wikipedia: https://en.wikipedia.org/wiki/Vim_(text_editor)
