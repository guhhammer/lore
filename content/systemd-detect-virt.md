# systemd-detect-virt

Detect execution in a virtualized environment.

## Usage
- `systemd-detect-virt --list` : list detectable virtualization technologies
- `systemd-detect-virt` : detect virtualization, print the result, and return a zero status code when running in a VM or a container, and a non-zero code otherwise
- `systemd-detect-virt -q` : silently check without printing anything
- `systemd-detect-virt -c` : only detect container virtualization
- `systemd-detect-virt -v` : only detect hardware virtualization
- `systemd-detect-virt -r` : detect whether in a `chroot` environment

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-detect-virt
