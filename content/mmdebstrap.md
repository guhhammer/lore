# mmdebstrap

Create a Debian chroot. Alternative to `debootstrap`.

## Usage
- `sudo mmdebstrap stable path/to/debian-root/` : create a Debian Stable directory chroot
- `mmdebstrap bookworm path/to/debian-bookworm.tar http://mirror.example.org/debian` : create a Debian Bookworm tarball chroot using a mirror
- `mmdebstrap sid path/to/debian-sid.tar --include=pkg1,pkg2` : create a Debian Sid tarball chroot with additional packages

## Categories
files, network

## Links
- More information: https://gitlab.mister-muffin.de/josch/mmdebstrap/
- tldr page: https://tldr.inbrowser.app/pages/linux/mmdebstrap
