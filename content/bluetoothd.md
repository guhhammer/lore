# bluetoothd

Daemon to manage bluetooth devices.

## Usage
- `bluetoothd` : start the daemon
- `bluetoothd -n` : start the daemon, logging to `stdout`
- `bluetoothd -f path/to/file` : start the daemon with a specific configuration file (defaults to `/etc/bluetooth/main.conf`)
- `bluetoothd -d` : start the daemon with verbose output to `stderr`
- `bluetoothd -dpath/to/file1:path/to/file2:...` : start the daemon with verbose output coming from specific files in the bluetoothd or plugins source

## Categories
system, hardware, files

## Links
- More information: https://manned.org/bluetoothd
- tldr page: https://tldr.inbrowser.app/pages/linux/bluetoothd
