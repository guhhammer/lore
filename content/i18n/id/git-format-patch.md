# git format-patch

Buat berkas-berkas `.patch` dari kumpulan komit Git. Dapat dipakai untuk mengirimkan perubahan/komit melalui surel/email.

## Usage
- `git format-patch origin` : buat suatu berkas `.patch` untuk mencatat seluruh komit yang belum dikirimkan (push) ke remote, menggunakan nama berkas otomatis
- `git format-patch revisi_1..revisi_2` : tampilkan isi berkas `.patch` menuju `stdout` yang mengandung perubahan antara dua revisi/komit
- `git format-patch -3` : tulis suatu berkas `.patch` yang mengandung segala perubahan dalam 3 komit terakhir

## Links
- More information: https://git-scm.com/docs/git-format-patch
