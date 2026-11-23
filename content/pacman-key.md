# pacman-key

Wrapper script for GnuPG used to manage pacman's keyring.

## Usage
- `sudo pacman-key --init` : initialize the `pacman` keyring
- `sudo pacman-key --populate` : add the default Arch Linux keys
- `pacman-key -l` : list keys from the public keyring
- `sudo pacman-key -a path/to/keyfile.gpg` : add the specified keys
- `sudo pacman-key -r "uid|name|email"` : receive a key from a key server
- `pacman-key -f "uid|name|email"` : print the fingerprint of a specific key
- `sudo pacman-key --lsign-key "uid|name|email"` : sign an imported key locally
- `sudo pacman-key -d "uid|name|email"` : remove a specific key

## See also
- pacman

## Categories
users, development

## Links
- More information: https://manned.org/pacman-key
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-key
