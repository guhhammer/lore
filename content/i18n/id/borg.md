# borg

Alat pencadangan bebas duplikat. Buat cadangan berkas lokal dan jarak jauh yang dapat dimuat sebagai sistem penyimpanan.

## Usage
- `borg init -e repokey|keyfile|none|... jalan/menuju/direktori_repo` : inisialisasikan suatu repositori (atau repositori lokal)
- `borg create --progress jalan/menuju/direktori_repo::Senin jalan/menuju/direktori_sumber` : cadangkan suatu direktori ke dalam repositori dengan nama arsip "Senin"
- `borg list jalan/menuju/direktori_repo` : tampilkan daftar seluruh arsip yang dikelola dalam suatu repositori
- `borg extract pengguna@host:jalan/menuju/direktori_repo::Senin jalan/menuju/direktori_target --exclude '*.ext'` : ekstrak isi suatu direktori dalam arsip "Senin" dalam suatu repositori jarak jauh, kecuali seluruh berkas bernama `*.ext`
- `borg prune --keep-within 7d --list jalan/menuju/direktori_repo` : pangkas suatu repositori dengan membuang kumpulan arsip yang dibuat lebih dari 7 hari lalu, dengan menampilkan daftar perubahannya
- `borg mount jalan/menuju/direktori_repo::Senin jalan/menuju/titik_muat` : muat suatu repositori sebagai sistem penyimpanan FUSE
- `borg create --help` : tampilkan informasi bantuan mengenai pembuatan arsip

## Links
- More information: https://borgbackup.readthedocs.io/en/stable/usage/general.html
