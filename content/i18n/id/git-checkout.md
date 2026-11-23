# git checkout

Periksa isi (checkout) cabang atau alamat ke direktori kerja.

## Usage
- `git checkout -b nama_cabang` : buat cabang baru, kemudian lihat isinya
- `git checkout -b nama_cabang referensi` : buat ke cabang baru berdasarkan referensi tertentu (misal cabang, remote, cabang remote, dan tag), kemudian lihat isinya
- `git checkout nama_cabang` : lihat isi suatu cabang lokal
- `git checkout -` : lihat kembali cabang yang terakhir kali dilihat sebelum cabang saat ini
- `git checkout -t nama_remote/nama_cabang` : lihat isi cabang yang bersumber dari sumber jauh (remote)
- `git checkout .` : singkirkan semua perubahan yang tidak masuk status stage pada direktori saat ini (lihat `git reset` untuk perintah yang lebih mirip undo)
- `git checkout jalan/menuju/berkas` : singkirkan perubahan yang tidak masuk status stage pada berkas
- `git checkout nama_cabang -- jalan/menuju/berkas` : ganti berkas pada direktori saat ini dengan versi pada cabang lain

## Links
- More information: https://git-scm.com/docs/git-checkout
