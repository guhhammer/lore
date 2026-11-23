# gbase32

This command is an alias of GNU `base32`.

## Usage
- `base32 path/to/file` : encode a file
- `base32 -w 0|76|... path/to/file` : wrap encoded output at a specific width (`0` disables wrapping)
- `base32 -d path/to/file` : decode a file
- `command | base32` : encode from `stdin`
- `command | base32 -d` : decode from `stdin`

## Categories
files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gbase32
