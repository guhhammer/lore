# git commit

Komit berkas ke dalam sebuah repositori.

## Usage
- `git commit -m "pesan"` : komit berkas bertahap ke repositori dengan sebuah pesan
- `git commit -F jalan/menuju/berkas_pesan_komit` : komit berkas bertahap dengan pesan yang disimpan dalam suatu berkas
- `git commit -a -m "pesan"` : ubah secara otomatis semua berkas yang dimodifikasi menjadi ke status stage dan menambahkan sebuah pesan
- `git commit -S id_kunci_gpg -m "pesan"` : komit berkas bertahap kemudian tandatangani komit tersebut menggunakan kunci GPG (atau kunci yang didefinisikan dalam berkas konfigurasi jika tidak didefinisikan)
- `git commit --amend` : ganti komit terakhir dengan perubahan yang ada di status stage saat ini
- `git commit jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : komit berkas tertentu (yang sudah di status stage)
- `git commit -m "pesan" --allow-empty` : buat komit kosong, tanpa berkas bertahap
- `git commit -m "pesan" -m "deskripsi_pesan"` : buat komit dengan sebuah pesan beserta deskripsi pesan

## Links
- More information: https://git-scm.com/docs/git-commit
