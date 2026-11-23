# update-grub2

Regenerates the GRUB boot menu file from the installed kernels and the system's settings.

## Usage
- `update-grub2` : rebuild the boot menu
- `update-grub2 -o /boot/grub/grub.cfg` : name the output file explicitly (normally the default)

## Notes
Needs root. Edit `/etc/default/grub` first, then run this for the change to take effect. A broken boot menu can stop the machine from starting.

## Categories
system, files
