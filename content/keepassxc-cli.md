# keepassxc-cli

Interface for KeePassXC.

## Usage
- `keepassxc-cli search path/to/database_file name` : search entries
- `keepassxc-cli ls path/to/database_file path/to/directory` : list the contents of a folder
- `keepassxc-cli add -g path/to/database_file entry_name` : add an entry with an auto-generated password
- `keepassxc-cli rm path/to/database_file entry_name` : delete an entry
- `keepassxc-cli clip path/to/database_file entry_name` : copy an entry's password to the clipboard
- `keepassxc-cli clip -t path/to/database_file entry_name` : copy a TOTP code to the clipboard
- `keepassxc-cli diceware -W 7` : generate a passphrase with 7 words
- `keepassxc-cli generate -lUns -L 16` : generate a password with 16 printable ASCII characters

## Categories
files, network, users

## Links
- More information: https://manned.org/keepassxc-cli
- tldr page: https://tldr.inbrowser.app/pages/common/keepassxc-cli
