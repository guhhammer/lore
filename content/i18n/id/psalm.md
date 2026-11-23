# psalm

Suatu alat analisis kode statis yang dapat dimanfaatkan untuk mencari kesalahan pada aplikasi berbasis PHP.

## Usage
- `psalm --init` : buat sebuah berkas konfigurasi Psalm baru
- `psalm` : periksa kesalahan pada direktori saat ini
- `psalm jalan/menuju/berkas_atau_direktori` : periksa kesalahan pada suatu direktori atau berkas
- `psalm --config jalan/menuju/psalm.xml` : periksa kesalahan menggunakan suatu berkas konfigurasi
- `psalm --show-info` : lampirkan penemuan-penemuan informasional ke dalam luaran hasil pengecekan
- `psalm --stats` : periksa suatu proyek dan tampilkan hasil statistika
- `psalm --threads 4` : periksa suatu proyek secara paralel denan 4 thread pemrosesan
