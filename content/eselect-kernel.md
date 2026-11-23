# eselect kernel

An `eselect` module for managing the `/usr/src/linux` symlink.

## Usage
- `eselect kernel list` : list available kernel symlink targets with their numbers
- `eselect kernel set name|number` : set the `/usr/src/linux` symlink by name or number from the `list` command
- `eselect kernel show` : show what the current kernel symlink points to
- `eselect kernel update` : set the kernel symlink to the currently running kernel

## Categories
system, files, development

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#Kernel
- tldr page: https://tldr.inbrowser.app/pages/linux/eselect-kernel
