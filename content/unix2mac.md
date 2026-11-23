# unix2mac

Change Unix-style line endings to macOS-style. Replaces LF with CR.

## Usage
- `unix2mac path/to/file` : change the line endings of a file
- `unix2mac -n path/to/file path/to/new_file` : create a copy with macOS-style line endings
- `unix2mac -i path/to/file` : display file information
- `unix2mac --keep-bom|add-bom|remove-bom path/to/file` : keep/add/remove Byte Order Mark

## See also
- unix2dos
- dos2unix
- mac2unix

## Categories
files, text

## Links
- More information: https://manned.org/unix2mac
- tldr page: https://tldr.inbrowser.app/pages/linux/unix2mac
