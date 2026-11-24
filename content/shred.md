# shred

Overwrite files to securely delete data.

## Usage
- `shred path/to/file` : overwrite a file
- `shred -v path/to/file` : overwrite a file and show progress on the screen
- `shred -z path/to/file` : overwrite a file, leaving zeros instead of random data
- `shred -n 25 path/to/file` : overwrite a file a specific number of times
- `shred -u path/to/file` : overwrite a file and remove it
- `shred -vzu -n 100 path/to/file` : overwrite a file 100 times, add a final overwrite with zeros, remove the file after overwriting it, and show verbose progress on the screen
- `shred --random-source /dev/urandom path/to/file` : overwrite a file using a specific source of random data

## Categories
danger, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/shred
- Wikipedia: https://en.wikipedia.org/wiki/Shred_(Unix)
