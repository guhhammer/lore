# ast-grep

Cari, periksa, dan tuilis ulang kode menggunakan pola pencarian AST.

## Usage
- `ast-grep run -p 'pola' jalan/menuju/berkas` : cari untuk suatu pola dalam kumpulan berkas
- `ast-grep run -p 'pola' -l python jalan/menuju/direktori` : cari untuk suatu pola dalam bahasa pemrograman tertentu
- `ast-grep run -p 'pola' -r 'kata_pengganti' jalan/menuju/berkas` : tulis ulang kode berdasarkan kriteria pola pencarian
- `ast-grep scan -r jalan/menuju/peraturan.yml jalan/menuju/direktori` : jalankan peraturan pencarian menurut suatu berkas konfigurasi
- `ast-grep run -p 'pola' -i jalan/menuju/direktori` : lakukan proses pencarian dan penulisan ulang kode secara interaktif
- `ast-grep run -h` : tampilkan bantuan untuk suatu subperintah

## Links
- More information: https://ast-grep.github.io/reference/cli.html
