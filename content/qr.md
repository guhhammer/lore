# qr

Generate QR codes in the terminal with ANSI VT-100 escape codes.

## Usage
- `qr "data"` : generate a QR code
- `qr --error-correction L|M|Q|H "data"` : specify the error correction level (defaults to `M`)
- `command | qr` : generate a QR code from the output of another command
- `qr "data" >path/to/file.png` : save the QR code as a PNG image

## See also
- qrencode

## Categories
shell

## Links
- More information: https://manned.org/qr
- tldr page: https://tldr.inbrowser.app/pages/common/qr
