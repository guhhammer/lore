# archinstall

Guided Arch Linux installer.

## Usage
- `archinstall` : start the interactive installer
- `archinstall --dry-run` : simulate an installation, going through the interactive installer, and generating a configuration file without installing
- `archinstall --advanced` : enable advanced options
- `archinstall --config path/to/config.json --creds path/to/credentials.json` : install using the specified configuration files
- `archinstall --config-url https://example.com/config.json --creds-url https://example.com/credentials.json` : install using configuration files from a remote server
- `archinstall --script minimal|only_hd` : install using the specified script

## Categories
network, text, files

## Links
- More information: https://archinstall.archlinux.page/installing/guided.html
- tldr page: https://tldr.inbrowser.app/pages/linux/archinstall
