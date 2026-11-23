# apt-get

Manajemen paket untuk Debian dan Ubuntu. Cari paket menggunakan `apt-cache`. Disarankan untuk menggunakan `apt` saat menggunakan perintah ini secara interaktif dalam Ubuntu versi 16.04 dan seterusnya.

## Usage
- `sudo apt-get update` : perbarui daftar paket yang tersedia beserta versinya (direkomendasikan untuk menggunakan perintah ini sebelum perintah `apt-get` lainnya)
- `sudo apt-get install paket` : pasang atau perbarui sebuah paket menuju versi terbaru
- `sudo apt-get remove paket` : hapus paket yang terpasang sebelumnya
- `sudo apt-get purge paket` : hapus paket yang terpasang sebelumnya, beserta seluruh berkas konfigurasi yang dibentuk oleh paket tersebut
- `sudo apt-get upgrade` : perbarui seluruh paket yang terpasang ke versi terbaru
- `sudo apt-get autoclean` : bersihkan repositori lokal, hapus berkas-berkas paket (`.deb`) yang sebelumnya gagal diunduh dan tak bisa diunduh kembali
- `sudo apt-get autoremove` : hapus seluruh paket yang tidak diperlukan kembali
- `sudo apt-get dist-upgrade` : perbarui kumpulan paket yang terinstal (mirip `upgrade`), namun hapus paket yang tidak dipakai kembali dan pasang paket tambahan untuk memenuhi dependensi baru

## Links
- More information: https://manned.org/apt-get.8
