# git changelog

Buat laporan riwayat perubahan (changelog) dari daftar komit dan tag yang terkandung dalam repositori Git. Bagian dari `git-extras`.

## Usage
- `git changelog` : buat atau mutakhirkan file `History.md` berisikan riwayat komit sejak tag komit Git terkini
- `git changelog -l` : tampilkan daftar komit pada versi saat ini
- `git changelog -l -s 2.1.0` : tampilkan daftar rentang komit yang dilakukan sejak tag komit `2.1.0` hingga komit terkini
- `git changelog -s 0.5.0 -f 1.0.0` : tampilkan, dengan format yang mudah dibaca manusia, daftar rentang komit antara tag `0.5.0` dan `1.0.0`
- `git changelog --start-commit 0b97430 -f 1.0.0` : tampilkan, dengan format yang mudah dibaca manusia, daftar rentang komit antara komit `0b97430` dan komit yang ditandai sebagai tag `1.0.0`
- `git changelog CHANGELOG.md` : gunakan `CHANGELOG.md` untuk menyimpan informasi daftar perubahan tersebut
- `git changelog -p` : hapus dan gantikan keseluruhan isi file perubahan dengan yang baru

## Links
- More information: https://manned.org/git-changelog
