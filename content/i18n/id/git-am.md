# git am

Gunakan perubahan dari file deskripsi perubahan (patch) untuk melakukan sebuah komit. Dapat digunakan untuk menerima komit melalui surel/email.

## Usage
- `git am jalan/menuju/berkas.patch` : gunakan dan komit perubahan dari file patch dalam direktori lokal
- `curl -L https://example.com/file.patch | git am` : gunakan dan komit perubahan dari file patch dari sumber dalam jaringan (online)
- `git am --abort` : batalkan proses perubahan yang dilakukan
- `git am --reject jalan/menuju/berkas.patch` : lakukan perubahan-perubahan dari file patch sebisa mungkin, dan tolak file patch jika proses tersebut gagal

## Links
- More information: https://git-scm.com/docs/git-am
