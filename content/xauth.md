# xauth

Edit and display the authorization information used in connecting to the X server.

## Usage
- `xauth -f path/to/file` : start interactive mode with a specific authority file (defaults to `~/.Xauthority`)
- `xauth info` : display information about the authority file
- `xauth list` : display authorization entries for all the displays
- `xauth add display_name protocol_name key` : add an authorization for a specific display
- `xauth remove display_name` : remove the authorization for a specific display
- `xauth extract - $DISPLAY` : print the authorization entry for the current display to `stdout`
- `cat path/to/file | xauth merge -` : merge the authorization entries from a specific file into the authorization database
- `xauth --help` : display help

## Categories
desktop, files

## Links
- More information: https://manned.org/xauth
- tldr page: https://tldr.inbrowser.app/pages/linux/xauth
