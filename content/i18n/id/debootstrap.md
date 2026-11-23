# debootstrap

Membuat sistem Debian dasar.

## Usage
- `sudo debootstrap stable jalan/ke/debian-root/ http://deb.debian.org/debian` : membuat sistem Debian stable didalam direktori `debian-root`
- `sudo debootstrap --variant=minbase stable jalan/ke/debian-root/` : membuat sistem minimal termasuk hanya paket yang diperlukan
- `sudo debootstrap focal jalan/ke/focal-root/ file:///jalan/ke/mirror/` : membuat sistem Ubuntu 20.04 didalam direktori `focal-root` dengan mirror lokal
- `sudo chroot jalan/ke/root` : berpindah ke sistem yang telah di bootstrap
- `ls /usr/share/debootstrap/scripts/` : memperlihatkan rilis Debian atau Ubuntu yang tersedia

## Links
- More information: https://wiki.debian.org/Debootstrap
