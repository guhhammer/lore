# nano

Text editor. An enhanced `pico` clone.

## Usage
- `nano path/to/file1 path/to/file2 ...` : open specific files, moving to the next file after closing the previous one
- `nano -I` : start the editor without using configuration files
- `nano +line,column path/to/file` : open a file and position the cursor at a specific line and column
- `nano -S path/to/file` : open a file and enable soft wrapping
- `nano -i path/to/file` : open a file and indent new lines to the previous line's indentation
- `nano -B path/to/file` : open a file and create a backup file (`path/to/file~`) on save
- `nano -R path/to/file` : open a file in restricted mode (i.e. don't read/write to files not specified on the command-line)
- `<Ctrl x>` : exit nano

## See also
- pico
- rnano

## Categories
editors

## Links
- More information: https://nano-editor.org/dist/latest/nano.html
- tldr page: https://tldr.inbrowser.app/pages/common/nano
- Wikipedia: https://en.wikipedia.org/wiki/GNU_nano
