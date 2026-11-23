# devfsadm

Perintah administrasi untuk `/dev`. Kelola namespace `/dev`.

## Usage
- `devfsadm -c disk` : pindai piringan baru
- `devfsadm -C -v` : bersihkan semua tautan /dev yang beruntai dan memindai perangkat baru
- `devfsadm -C -v -n` : dry-run - luaran yang akan dirubah tapi tanpa membuat modifikasi

## Links
- More information: https://www.unix.com/man-page/sunos/1m/devfsadm
