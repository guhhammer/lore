# mimetype

Automatically determine the MIME type of a file.

## Usage
- `mimetype path/to/file` : print the MIME type of a given file
- `mimetype --brief path/to/file` : display only the MIME type, and not the filename
- `mimetype --describe path/to/file` : display a description of the MIME type
- `command | mimetype --stdin` : determine the MIME type of `stdin` (does not check a filename)
- `mimetype --debug path/to/file` : display debug information about how the MIME type was determined
- `mimetype --all path/to/file` : display all the possible MIME types of a given file in confidence order
- `mimetype --language path/to/file` : explicitly specify the 2-letter language code of the output

## Categories
files, desktop

## Links
- More information: https://manned.org/mimetype
- tldr page: https://tldr.inbrowser.app/pages/linux/mimetype
