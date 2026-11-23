# gbase64

This command is an alias of GNU `base64`.

## Usage
- `base64 path/to/file` : encode a file
- `base64 -w 0|76|... path/to/file` : wrap encoded output at a specific width (`0` disables wrapping)
- `base64 -d path/to/file` : decode a file
- `command | base64` : encode from `stdin`
- `command | base64 -d` : decode from `stdin`

## Categories
files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gbase64
