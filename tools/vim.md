# vim

vim (and neovim, which uses the same keys) is a modal editor: keys do different things in normal mode and in insert mode. This is the survival kit.

## Tasks
### Modes
- `i` : insert before the cursor (type text)
- `a` : insert after the cursor
- `o` : open a new line below and insert
- `Esc` : back to normal mode
- `v` : select characters; `V` selects whole lines
### Save and quit
- `:w` : save
- `:q` : quit
- `:wq` : save and quit (also `ZZ`)
- `:q!` : quit and throw away changes
- `:w !sudo tee %` : save a file you opened without permission
### Move
- `h j k l` : left, down, up, right
- `w` : next word; `b` goes back a word
- `0` : start of the line; `$` end of the line
- `gg` : top of the file; `G` bottom
- `:42` : go to line 42
### Edit
- `x` : delete a character
- `dd` : delete (cut) the line
- `yy` : copy the line
- `p` : paste after the cursor
- `cw` : change a word
- `u` : undo
- `Ctrl+r` : redo
- `.` : repeat the last change
### Search and replace
- `/TEXT` : search forward; `n` next, `N` previous
- `:%s/OLD/NEW/g` : replace everywhere in the file
- `:%s/OLD/NEW/gc` : replace, asking each time
- `:noh` : stop highlighting matches
### Windows and files
- `:split` : split the screen in two
- `Ctrl+w w` : move between splits
- `:set number` : show line numbers

## Notes
- Stuck? Press `Esc` a couple of times, then type `:q!` and Enter to leave without saving.
- A number before a key repeats it: `3dd` deletes three lines.
- Neovim reads `~/.config/nvim/init.lua`; vim reads `~/.vimrc`.
- `vimtutor` is a 30-minute guided lesson that comes with vim.

## See also
- nvim
- nano
- vimtutor
- tmux

## Links
- Help: https://vimhelp.org
- Neovim docs: https://neovim.io/doc/

## Categories
editors
