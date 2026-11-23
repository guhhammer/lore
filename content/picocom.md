# picocom

Minimal program to emulate serial consoles.

## Usage
- `sudo picocom /dev/ttyXYZ` : connect to a serial console with the default baud rate of 9600
- `sudo picocom /dev/ttyXYZ -b baud_rate` : connect to a serial console with a specified baud rate
- `sudo picocom /dev/ttyXYZ --imap lfcrlf` : map special characters (e.g. `LF` to `CRLF`)
- `<Ctrl a><Ctrl x>` : exit picocom
- `picocom -h` : display help

## See also
- minicom
- cu
- tio

## Categories
users

## Links
- More information: https://manned.org/picocom
- tldr page: https://tldr.inbrowser.app/pages/common/picocom
