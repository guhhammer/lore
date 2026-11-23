# base64

Encode or decode file or `stdin` to/from Base64, to `stdout`.

## Usage
- `base64 path/to/file` : encode a file
- `base64 -w 0|76|... path/to/file` : wrap encoded output at a specific width (`0` disables wrapping)
- `base64 -d path/to/file` : decode a file
- `command | base64` : encode from `stdin`
- `command | base64 -d` : decode from `stdin`

## Categories
files

## Links
- More information: https://manned.org/base64
- tldr page: https://tldr.inbrowser.app/pages/common/base64
