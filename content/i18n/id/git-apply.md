# git apply

Gunakan perubahan dari file deskripsi perubahan (patch) kepada indeks perubahan tanpa mencatat sebuah komit.

## Usage
- `git apply -v jalan/menuju/berkas` : tampilkan informasi lengkap (mode verbose) atas proses perubahan yang sedang dilakukan
- `git apply --index jalan/menuju/berkas` : gunakan patch dan tambahkan file yang diubah ke dalam indeks perubahan
- `curl -L https://example.com/file.patch | git apply` : gunakan perubahan dari file patch dari sumber dalam jaringan (online)
- `git apply --stat --apply jalan/menuju/berkas` : tampilkan informasi statistik perbedaan (diffstat) setelah melakukan perubahan menurut file patch
- `git apply -R jalan/menuju/berkas` : batalkan perubahan yang dilakukan melalui file patch
- `git apply --cached jalan/menuju/berkas` : simpan hasil perubahan ke dalam indeks perubahan tanpa merubah susunan file/direktori dalam direktori kerja saat ini

## Links
- More information: https://git-scm.com/docs/git-apply
