# qrencode

QR Code generator. Supports PNG and EPS.

## Usage
- `qrencode -o path/to/output_file.png string` : convert a string to a QR code and save to an output file
- `qrencode -o path/to/output_file.png -M string` : convert a string to a micro QR code and save to an output file
- `qrencode -o path/to/output_file.png -r path/to/input_file` : convert an input file to a QR code and save to an output file
- `qrencode -t ansiutf8 string` : convert a string to a QR code and print it in terminal
- `qrencode -o path/to/output_file.png -l l|m|q|h string` : convert a string to a QR code with a specific error correction level and save to an output file
- `echo string | qrencode -t utf8` : convert input from pipe to a QR code and print it in terminal

## See also
- qr

## Categories
files, text

## Links
- More information: https://manned.org/qrencode
- tldr page: https://tldr.inbrowser.app/pages/common/qrencode
