# kak

Kakoune, a mode-based code editor implementing the "multiple selections" paradigm. Data can be selected and simultaneously edited in different locations, using multiple selections; users can also connect to the same session for collaborative editing.

## Usage
- `kak path/to/file` : open a file and enter normal mode, to execute commands
- `<i>` : enter insert mode from normal mode, to write text into the file
- `<Esc>` : escape insert mode, to go back to normal mode
- `%sstring1<Enter>cstring2<Esc>` : replace all instances of `string1` in the current file with `string2`
- `<Space>` : unselect all secondary selections, and keep only the main one
- `/\d+<Enter>N` : search for numbers and select the first two
- `<!>cat path/to/file<Enter>` : insert the contents of a file
- `<:>w<Enter>` : save the current file

## Categories
files, editors, users

## Links
- More information: https://manned.org/kak
- tldr page: https://tldr.inbrowser.app/pages/common/kak
