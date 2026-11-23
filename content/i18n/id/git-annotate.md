# git annotate

Tampilkan kode hash serta pelaku komit terakhir pada setiap baris suatu file teks. Lihat juga `git blame`, yang lebih disarankan daripada `git annotate`. Perintah `git annotate` disediakan bagi pengguna yang telah familiar pada sistem manajemen versi lainnya.

## Usage
- `git annotate jalan/menuju/berkas` : tampilkan file teks beserta informasi nama pelaku dan kode hash komit terakhir pada akhir setiap baris teks
- `git annotate -e jalan/menuju/berkas` : tampilkan file dengan informasi komit menggunakan alamat surel/email daripada nama pelaku
- `git annotate -L :regex jalan/menuju/berkas` : tampilkan hanya baris-baris teks yang memenuhi kriteria `regex`

## Links
- More information: https://git-scm.com/docs/git-annotate
