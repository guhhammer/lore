# pacman-query

Kegunaan manajer paket Arch Linux.

## Usage
- `pacman -Q` : tampilkan daftar paket yang diinstal beserta versinya
- `pacman -Qe` : tampilkan daftar paket yang diinstal beserta versinya secara eksplisit
- `pacman -Qo namafile` : temukan paket mana yang memiliki file
- `pacman -Qi paket` : tampilkan informasi paket yang diinstal
- `pacman -Ql paket` : tampilkan daftar file yang dimiliki oleh paket
- `pacman -Qdtq` : tampilkan daftar paket yatim piatu (diinstal sebagai dependensi namun tidak dibutuhkan oleh paket apa pun)
- `pacman -Qm` : tampilkan daftar paket yang diinstal tidak ditemukan di tempat penyimpanan
- `pacman -Qu` : tampilkan daftar paket usang

## Links
- More information: https://manned.org/pacman.8
