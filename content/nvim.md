# nvim

Neovim, a programmer's text editor based on Vim, provides several modes for different kinds of text manipulation. Pressing `<i>` in normal mode enters insert mode. `<Esc>` or `<Ctrl c>` goes back to normal mode, which doesn't allow regular text insertion.

## Usage
- `nvim path/to/file` : open a file
- `<Esc><i>` : enter text editing mode (insert mode)
- `<Esc><y><y>|<d><d>` : copy ("yank") or cut ("delete") the current line (paste it with `<p>`)
- `<Esc><u>` : enter normal mode and undo the last operation
- `<Esc></>search_pattern<Enter>` : search for a pattern in the file (press `<n>`/`<N>` to go to next/previous match)
- `<Esc><:>%s/regex/replacement/g<Enter>` : perform a `regex` substitution in the whole file
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : enter normal mode and save (write) the file, and quit
- `<Esc><:>q!<Enter>` : quit without saving

## See also
- vim
- vimtutor
- vimdiff

## Categories
editors

## Links
- More information: https://neovim.io/
- tldr page: https://tldr.inbrowser.app/pages/common/nvim
