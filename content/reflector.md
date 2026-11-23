# reflector

Arch script to fetch and sort mirrorlists.

## Usage
- `sudo reflector --sort rate --save /etc/pacman.d/mirrorlist` : get all mirrors, sort for download speed, and save them
- `reflector -c Germany -p https` : only get German HTTPS mirrors
- `reflector -l 10` : only get the 10 recently synced mirrors
- `sudo reflector @/etc/xdg/reflector/reflector.conf` : use a configuration file to fetch mirrors
- `reflector -h` : display help

## Categories
network, development

## Links
- More information: https://manned.org/reflector
- tldr page: https://tldr.inbrowser.app/pages/linux/reflector
