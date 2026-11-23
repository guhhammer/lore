# git fetch

Unduh kumpulan objek dan referensi dari suatu repositori jarak jauh.

## Usage
- `git fetch` : dapatkan perubahan-perubahan terkini dari repositori bersumber hulu jarak jauh (remote upstream) yang ditentukan sebagai default (bila ditentukan)
- `git fetch nama_remote` : dapatkan daftar cabang-cabang baru dari sumber hulu jarak jauh yang ditentukan
- `git fetch --all` : dapatkan daftar perubahan terkini dari seluruh repositori hulu jarak jauh
- `git fetch -t` : dapatkan juga daftar tag yang terdaftar dalam repositori hulu jarak jauh
- `git fetch -p` : hapus referensi-referensi lokal menuju cabang-cabang repositori hulu yang telah dihapus secara jarak jauh
- `git fetch --deepen 2` : perdalam tingkat kedalaman komit dalam cabang saat ini (jika menggunakan shallow clone) dengan 2 komit
- `git fetch origin main:main` : perbarui cabang `main` tanpa mengalihkan isi direktori kerja repositori lokal menuju versi cabang tersebut (setara dengan `git pull`)

## Links
- More information: https://git-scm.com/docs/git-fetch
