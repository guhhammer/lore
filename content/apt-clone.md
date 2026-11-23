# apt-clone

Clone/back up/restore the package state of a Debian-based system.

## Usage
- `apt-clone clone path/to/directory` : clone the package state of the current system into a specified directory
- `apt-clone clone --destination path/to/backup.tar.gz` : create a clone file (`.tar.gz`) for backup purposes
- `apt-clone restore path/to/backup.tar.gz` : restore the package state from a clone file
- `apt-clone info path/to/backup.tar.gz` : show information about a clone file (e.g., the release, architecture)
- `apt-clone restore path/to/backup.tar.gz --destination path/to/restore` : check if the clone file can be restored on the current system

## Categories
files, archives, packages

## Links
- More information: https://github.com/mvo5/apt-clone
- tldr page: https://tldr.inbrowser.app/pages/linux/apt-clone
