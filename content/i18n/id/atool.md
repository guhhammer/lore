# atool

Suatu skrip untuk mengatur isi berkas arsip dengan berbagai format. `atool` menggunakan beragam program pengarsip eksternal namun menyediakan antarmuka baris perintah yang sama untuk melihat isi, mengekstrak, membangun, dan mengatur arsip.

## Usage
- `atool -l jalan/menuju/arsip.zip` : tampilkan isi berkas di dalam suatu arsip
- `atool -x arsip.tar.gz` : ekstrak isi arsip (akan membuat subdirektori jika diperlukan)
- `atool -X jalan/menuju/direktori_luaran arsip.rar` : ekstrak isi arsip menuju suatu direktori
- `atool -c arsip.tar jalan/menuju/berkas_dalam_arsip.txt` : tampilkan isi suatu berkas yang tersimpan dari arsip menuju `stdout` (seperti `cat`)
- `atool -a arsip_baru.zip jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : buat suatu berkas arsip untuk kumpulan berkas dan/atau direktori yang ditentukan
- `atool -l -p arsip_besar.tar.bz2` : tampilkan daftar isi berkas arsip dan kirim luaran program menuju program penampil teks dengan manajemen halaman
- `atool -x -e arsip1.zip arsip2.tar.gz *.rar` : ekstrak beberapa arsip dengan sekali jalan (setiap arsip dapat diekstrak menuju masing-masing subdirektori bila dibutuhkan)
- `atool -r arsip_lama.tar.gz arsip_baru.tar.7z` : kemas ulang suatu arsip dari suatu format menuju format lain (misalnya, `.tar.gz` menuju `.tar.7z`)

## Links
- More information: https://manned.org/atool
