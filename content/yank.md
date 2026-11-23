# yank

Read input from `stdin` and display a selection interface that allows a field to be selected and copied to the clipboard.

## Usage
- `sudo dmesg | yank` : yank using the default delimiters (\f, \n, \r, \s, \t)
- `sudo dmesg | yank -l` : yank an entire line
- `echo hello=world | yank -d =` : yank using a specific delimiter
- `ps ux | yank -g "[0-9]+"` : only yank fields matching a specific pattern

## Categories
desktop, network

## Links
- More information: https://manned.org/yank
- tldr page: https://tldr.inbrowser.app/pages/common/yank
