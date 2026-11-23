# eselect profile

An `eselect` module for managing the `/etc/portage/make.profile` symlink, which sets the system profile.

## Usage
- `eselect profile list` : list available profile symlink targets with their numbers
- `eselect profile set name|number` : set the `/etc/portage/make.profile` symlink by name or number from the `list` command
- `eselect profile show` : show the current system profile

## Categories
files, development, system

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#Profile
- tldr page: https://tldr.inbrowser.app/pages/linux/eselect-profile
