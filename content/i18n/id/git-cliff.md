# git cliff

Suatu pembuat teks daftar perubahan (changelog) yang mudah diatur.

## Usage
- `git cliff >CHANGELOG.md` : buat suatu pesan changelog dari seluruh komit dalam suatu repositori Git, kemudian simpan ke dalam `CHANGELOG.md`
- `git cliff -l` : buat suatu pesan changelog atas komit-komit yang dimulai sejak tag terkini dan tampilkan pesan menuju `stdout`
- `git cliff --current` : buat suatu pesan changelog atas komit-komit yang masuk ke dalam tag saat ini (gunakan `git checkout` atas suatu tag sebelum melakukan ini)
- `git cliff -u` : buat suatu pesan changelog atas komit-komit yang tidak termasuk dalam tag apapun
- `git cliff -i` : tulis berkas konfigurasi bawaan menuju `cliff.toml` dalam direktori saat ini

## Links
- More information: https://git-cliff.org/docs/usage/args/
