# systemctl bind

Ephemerally bind-mount a file or directory from the host into a unit's mount namespace.

## Usage
- `systemctl bind unit /path/to/host_directory` : bind-mount a host path into the same location inside the unit
- `systemctl bind unit /path/to/host_directory /path/to/unit_directory` : bind-mount a host path into a different location inside the unit
- `systemctl bind unit /path/to/host_directory --read-only` : bind-mount a path as read-only inside the unit
- `systemctl bind unit /path/to/host_directory /path/to/unit_directory --mkdir` : create the destination path inside the unit before binding

## Categories
files, disks, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-bind
