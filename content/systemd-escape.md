# systemd-escape

Escape strings for usage in systemd unit names.

## Usage
- `systemd-escape text` : escape the given text
- `systemd-escape -u text` : reverse the escaping process
- `systemd-escape -p text` : treat the given text as a path
- `systemd-escape --suffix suffix text` : append the given suffix to the escaped text
- `systemd-escape --template template text` : use a template and inject the escaped text

## Categories
system, text, monitoring

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-escape.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-escape
