# debootstrap

Create a basic Debian system.

## Usage
- `sudo debootstrap stable path/to/debian-root/ http://deb.debian.org/debian` : create a Debian stable release system inside the `debian-root` directory
- `sudo debootstrap --variant=minbase stable path/to/debian-root/` : create a minimal system including only required packages
- `sudo debootstrap sid path/to/sid-root/ file:///path/to/mirror/` : create a Debian Unstable system inside the `sid-root` directory with a local mirror
- `sudo chroot path/to/root` : switch to a bootstrapped system
- `ls /usr/share/debootstrap/scripts/` : list available releases

## Categories
files, users, network

## Links
- More information: https://wiki.debian.org/Debootstrap
- tldr page: https://tldr.inbrowser.app/pages/linux/debootstrap
