# at

Jalankan kumpulan perintah pada lain waktu. Hasil penugasan perintah akan dikirimkan menuju surel pengguna.

## Usage
- `at now + 5 minutes` : buat perintah secara interaktif untuk dijalankan dalam 5 menit ke depan (gunakan `<Ctrl d>` jika selesai menulis)
- `at hh:mm` : buat perintah secara interaktif dan jalankan pada waktu tertentu
- `echo "perintah" | at 1000` : jalankan perintah yang dimasukkan ke dalam `stdin` pada hari ini pukul 10.00 pagi
- `at -f jalan/menuju/berkas 9:30 PM Tue` : jalankan perintah yang diatur dalam suatu berkas pada hari Selasa berikutnya
- `at -l` : tampilkan seluruh pekerjaan terjadwal untuk pengguna saat ini (sama dengan perintah `atq`)
- `at -c nomor_pekerjaan` : lihat rincian suatu pekerjaan

## Links
- More information: https://manned.org/at
