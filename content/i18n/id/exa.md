# exa

Pengganti `ls` (Tampilkan isi direktori) yang modern.

## Usage
- `exa -1` : tampilkan daftar berkas, satu berkas per baris
- `exa -a` : tampilkan daftar seluruh berkas, termasuk berkas-berkas tersembunyi
- `exa -l -a` : tampilkan daftar seluruh berkas dengan format informasi panjang (perizinan, pemilik, ukuran, dan tanggal modifikasi)
- `exa -r -s size` : tampilkan daftar seluruh berkas dengan berkas terbesar terlebih dahulu
- `exa -l -T -L 3` : tampilkan daftar berkas dalam format pohon (tree) hingga tingkat kedalaman ketiga
- `exa -l -s modified` : tampilkan daftar berkas yang diurutkan menurut tanggal modifikasi (terlawas terlebih dahulu)
- `exa -l -h --icons --git` : tampilkan daftar berkas dengan informasi header, ikon, dan status berkas dalam Git
- `exa --git-ignore` : jangan tampilkan daftar berkas yang terdaftar dalam `.gitignore`

## Links
- More information: https://github.com/ogham/exa#command-line-options
