# nala

Package management utility with better formatting. Front-end for the `python-apt` API.

## Usage
- `sudo nala install package` : install a package, or update it to the latest available version
- `sudo nala remove package` : remove a package
- `nala purge package` : remove a package and its configuration files
- `nala search "pattern"` : search package names and descriptions using a word, `regex` (default), or `glob`
- `sudo nala upgrade` : update the list of available packages and upgrade the system
- `sudo nala autoremove` : remove all unused packages and dependencies from your system
- `sudo nala fetch` : fetch fast mirrors to improve download speeds
- `nala history` : display the history of all transactions

## Categories
packages, users, network

## Links
- More information: https://gitlab.com/volian/nala
- tldr page: https://tldr.inbrowser.app/pages/linux/nala
