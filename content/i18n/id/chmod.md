# chmod

Mengubah hak akses pengguna suatu file atau direktori.

## Usage
- `chmod u+x jalan/menuju/berkas` : berikan pengguna pembuat file hak untuk mengeksekusinya (misal: sebagai script)
- `chmod u+rw jalan/menuju/berkas_atau_direktori` : berikan hak kepada pengguna untuk membaca (read) dan menulis (write) suatu file atau direktori
- `chmod g-x jalan/menuju/berkas` : hentikan hak akses terhadap grup untuk mengeksekusi suatu file
- `chmod a+rx jalan/menuju/berkas` : berikan semua pengguna hak untuk membaca dan mengeksekusi suatu file
- `chmod o=g jalan/menuju/berkas` : berikan hak-hak akses suatu file yang sama dari grup kepada pengguna di luar grup (others)
- `chmod o= jalan/menuju/berkas` : hentikan semua hak akses suatu file
- `chmod -R g+w,o+w jalan/menuju/direktori` : berikan hak tulis suatu direktori bagi grup dan lainnya secara Rekursif (termasuk seluruh file yang terkandung di dalamnya)
- `chmod -R a+rX jalan/menuju/direktori` : berikan hak semua pengguna untuk membaca seluruh file dan mengeksekusi para sub-direktori dalam suatu direktori
