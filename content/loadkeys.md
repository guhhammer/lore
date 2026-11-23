# loadkeys

Load the kernel keymap for the console.

## Usage
- `sudo loadkeys en|de|fi|dvorak|defkeymap|...` : load a specific keyboard layout for the current console
- `sudo loadkeys -d` : load a default keymap
- `loadkeys -m en|de|fi|...` : print the kernel source table of a keymap to `stdout`
- `loadkeys -b en|de|fi|...` : print the binary format of a keymap to `stdout`
- `loadkeys -p en|de|fi|...` : search and parse keymap without action
- `command | sudo loadkeys -q` : load a keymap from `stdin`, suppressing all output
- `sudo loadkeys -C /dev/ttyN uk` : set a keymap for a specific console
- `sudo loadkeys -C /dev/ttyN /path/to/file` : load a keymap from the specified file for the console

## See also
- localectl

## Categories
monitoring, users, system

## Links
- More information: https://manned.org/loadkeys
- tldr page: https://tldr.inbrowser.app/pages/linux/loadkeys
