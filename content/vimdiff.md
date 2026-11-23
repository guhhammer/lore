# vimdiff

Open up two or more files in vim and show the differences between them.

## Usage
- `vimdiff path/to/file1 path/to/file2` : open two files and show the differences
- `<Ctrl w><h>|<l>` : move the cursor to the window on the left|right
- `<[><c>` : jump to the previous difference
- `<]><c>` : jump to the next difference
- `<d><o>` : copy the highlighted difference from the other window to the current window
- `<d><p>` : copy the highlighted difference from the current window to the other window
- `<:>diffupdate` : update all highlights and folds
- `<z><a>` : toggle the highlighted code fold

## See also
- vim
- vimtutor
- nvim

## Categories
desktop, files, editors

## Links
- More information: https://www.vim.org/
- tldr page: https://tldr.inbrowser.app/pages/common/vimdiff
