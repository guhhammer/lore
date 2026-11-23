# hx

Helix, a post-modern text editor, provides several modes for different kinds of text manipulation. Pressing `<i>` enters insert mode. `<Esc>` enters normal mode, which enables the use of Helix commands.

## Usage
- `hx path/to/file` : open a file
- `hx --vsplit path/to/file1 path/to/file2 ...` : open files and show them one next each other
- `hx --tutor` : show the tutorial to learn Helix (or access it within Helix by pressing `<Esc>` and typing `<:>tutor<Enter>`)
- `<:>theme theme_name` : change the Helix theme
- `<:>wq<Enter>` : save and quit
- `<:>q!<Enter>` : force-quit without saving
- `<u>|<U>` : undo/Redo the last operation
- `</>search_pattern<Enter>` : search for a pattern in the file (press `<n>`/`<N>` to go to next/previous match)

## Categories
text, editors, files

## Links
- More information: https://manned.org/man/debian-forky/hx
- tldr page: https://tldr.inbrowser.app/pages/common/hx
