# ctrlaltdel

Utility to control what happens when `<Ctrl Alt Del>` is pressed.

## Usage
- `ctrlaltdel` : get current setting
- `sudo ctrlaltdel hard` : set `<Ctrl Alt Del>` to reboot immediately, without any preparation
- `sudo ctrlaltdel soft` : set `<Ctrl Alt Del>` to reboot "normally", giving processes a chance to exit first (send `SIGINT` to PID1)

## Links
- More information: https://manned.org/ctrlaltdel
- tldr page: https://tldr.inbrowser.app/pages/linux/ctrlaltdel
