# base32

Encode or decode file or `stdin` to/from Base32, to `stdout`.

## Usage
- `base32 path/to/file` : encode a file
- `base32 -w 0|76|... path/to/file` : wrap encoded output at a specific width (`0` disables wrapping)
- `base32 -d path/to/file` : decode a file
- `command | base32` : encode from `stdin`
- `command | base32 -d` : decode from `stdin`

## Categories
files

## Links
- More information: https://manned.org/base32
- tldr page: https://tldr.inbrowser.app/pages/common/base32
