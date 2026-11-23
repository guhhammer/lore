# grep

Cari pola teks tertentu pada kumpulan berkas menggunakan kata pencarian ekspresi reguler (`regex`).

## Usage
- `grep "pola_pencarian" jalan/menuju/berkas` : cari suatu berkas untuk teks yang mengikuti pola pencarian tertentu
- `grep -F "teks_spesifik" jalan/menuju/berkas` : cari berkas untuk teks string tertentu secara spesifik (dengan menghentikan pencarian berbasis ekspresi reguler)
- `grep -rnI "pola_pencarian" jalan/menuju/direktori` : cari seluruh berkas selain berkas format biner di dalam suatu direktori secara rekursif (termasuk berkas-berkas di dalam subdirektori) dengan menunjukkan nomor barisan di mana pola tersebut ditemukan
- `grep -Ei "pola_pencarian" jalan/menuju/berkas` : gunakan sintaks ekspresi reguler tingkat lanjut (mendukung `?`, `+`, `{}`, `()`, dan `|`), dalam mode case-insensitive (tanpa menghiraukan perbedaan antara huruf kapital dan kecil)
- `grep --context|--before-context|--after-context 3 "pola_pencarian" jalan/menuju/berkas` : cetak 3 baris konteks isi berkas pada sekitar, sebelum, atau sesudah setiap hasil pencarian
- `grep -Hn --color=always "pola_pencarian" jalan/menuju/berkas` : cetak nama berkas dan nomor baris di mana pola tersebut ditemukan dalam format teks berwarna
- `grep -o "pola_pencarian" jalan/menuju/berkas` : cari untuk barisan teks yang memenuhi kriteria pada pola pencarian, dan hanya cetak bagian teks yang memenuhi pola
- `cat jalan/menuju/berkas | grep -v "pola_pencarian"` : cari `stdin` untuk barisan teks yang tidak memenuhi kriteria pada pola pencarian

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
