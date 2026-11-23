# systemd-cgls

Show the contents of the selected Linux control group hierarchy in a tree.

## Usage
- `systemd-cgls` : display the whole control group hierarchy on your system
- `systemd-cgls cpu|memory|io` : display a control group tree of a specific resource controller
- `systemd-cgls -u unit1 unit2 ...` : display the control group hierarchy of one or more systemd units

## Categories
system, users, desktop

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-cgls.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-cgls
