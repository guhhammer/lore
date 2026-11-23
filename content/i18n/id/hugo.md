# hugo

Penghasil situs web statis berbasis template. Menggunakan modul, komponen dan tema.

## Usage
- `hugo new site jalan/menuju/website` : buat sebuah proyek situs web Hugo baru
- `hugo new theme nama_tema` : buat sebuah proyek tema Hugo baru (tema juga dapat diunduh dari <https://themes.gohugo.io/>)
- `hugo new nama_bagian/nama_halaman` : buat sebuah halaman situs web baru
- `hugo` : bangun situs web dari direktori sumber menuju direktori `./public`
- `hugo -D` : bangun situs web termasuk halaman yang ditandai sebagai "draft"
- `hugo server --bind ip_lokal -b http://ip_lokal` : bangun situs web dengan untuk dijalankan pada alamat IP lokal
- `hugo -d alamat/tujuan` : bangun situs web menuju direktori yang ditentukan
- `hugo server` : bangun situs web dan jalankan peladen (server) untuk menyajikannya, dengan memuat ulang saat terdapat halaman yang berubah

## Links
- More information: https://gohugo.io/commands/
