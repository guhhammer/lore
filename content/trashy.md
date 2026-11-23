# trashy

An alternative to `rm` and `trash-cli` written in Rust.

## Usage
- `trash path/to/file` : move a specific file to the trash
- `trash path/to/file1 path/to/file2 ...` : move specific files to the trash
- `trash list` : list items in the trash
- `trash restore file` : restore a specific file from the trash
- `trash empty file` : remove a specific file from the trash
- `trash restore --all` : restore all files from the trash
- `trash empty --all` : remove all files from the trash

## Categories
files

## Links
- More information: https://github.com/oberblastmeister/trashy#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/trashy
