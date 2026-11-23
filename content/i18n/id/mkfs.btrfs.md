# mkfs.btrfs

Pasang strategi manajemen berkas terhadap perangkat penyimpanan menggunakan sistem BTRFS. Default ke `raid1`, yang menyatakan 2 salinan sebuah blok data disebar ke 2 perangkat yang berbeda.

## Usage
- `sudo mkfs.btrfs /dev/sdXY` : buat instalasi BTRFS baru terhadap suatu partisi kosong
- `sudo mkfs.btrfs -m single -d single /dev/sdX` : buat instalasi BTRFS di suatu perangkat penyimpanan
- `sudo mkfs.btrfs -m raid1 -d raid1 /dev/sdX /dev/sdY /dev/sdZ ...` : buat instalasi BTRFS yang mencakup beberapa perangkat penyimpanan dengan raid1
- `sudo mkfs.btrfs -L "label" /dev/sdX /dev/sdY ...` : tentukan label nama untuk pemasangan instalasi manajemen berkas
- `sudo mkfs.btrfs -f /dev/sdX` : tangguhkan instalasi sistem manajemen berkas saat ini jika ditemukan dalam perangkat tersebut

## Links
- More information: https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html
