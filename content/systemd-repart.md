# systemd-repart

Automatically grow and add partitions. Grows and adds partitions based on the configuration files described in repart.d. Does not automatically resize filesystem on partition. See systemd-growfs to extend filesystem.

## Usage
- `systemd-repart` : grow the root partition (/) to all available disk space
- `systemd-repart --dry-run yes` : simulate repartitioning, showing the changes without applying them
- `systemd-repart --size 10G --root /` : grow root partition size to 10 gigabytes

## Categories
disks, system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-repart.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-repart
