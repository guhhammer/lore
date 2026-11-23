# apt

Manajer paket untuk distribusi Linux berbasis Debian. Pengganti `apt-get` yang direkomendasikan ketika digunakan secara interaktif di Ubuntu versi 16.04 atau yang lebih baru.

## Usage
- `sudo apt update` : perbarui daftar paket yang tersedia dan versinya (direkomendasikan untuk menggunakan perintah ini sebelum perintah `apt` lainnya.)
- `apt search paket` : cari paket yang tersedia dengan nama atau deskripsi tertentu
- `apt list paket` : cari paket yang tersedia dengan nama saja (mendukung kata kunci wildcard seperti `*`)
- `apt show paket` : tampilkan informasi tentang suatu paket
- `sudo apt install paket` : pasang atau perbarui sebuah paket menuju versi terbaru
- `sudo apt remove paket` : hapus paket yang terpasang sebelumnya (gunakan `sudo apt purge` untuk sekaligus menghapus berkas konfigurasi yang dibentuk oleh paket tersebut)
- `sudo apt upgrade` : perbarui seluruh paket yang terpasang ke versi terbaru
- `apt list -i` : tampilkan daftar paket yang telah terpasang

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
