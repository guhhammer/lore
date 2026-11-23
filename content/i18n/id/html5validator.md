# html5validator

Lakukan proses validasi sintaks terhadap suatu berkas HTML5.

## Usage
- `html5validator jalan/menuju/berkas` : lakukan proses validasi terhadap suatu berkas
- `html5validator --root jalan/menuju/direktori` : lakukan validasi terhadap seluruh berkas HTML pada suatu direktori
- `html5validator --show-warnings jalan/menuju/berkas` : tampilkan seluruh pesan peringatan (warning) dan galat (error)
- `html5validator --root jalan/menuju/direktori --match "*.html *.php"` : lakukan proses validasi terhadap kumpulan berkas yang memenuhi `glob` kriteria nama berkas
- `html5validator --root jalan/menuju/direktori --blacklist "node_modules vendor"` : jangan lakukan validasi terhadap nama-nama berkas tertentu
- `html5validator --format gnu|xml|json|text jalan/menuju/berkas` : tampilkan laporan hasil analisa dalam format tertentu
- `html5validator --root jalan/menuju/direktori --log debug|info|warning` : tampilkan log validasi dalam tingkat verbositas tertentu

## Links
- More information: https://github.com/svenkreiss/html5validator
