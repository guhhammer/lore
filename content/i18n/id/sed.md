# sed

Mengedit teks secara skrip.

## Usage
- `perintah | sed 's/apple/mango/g'` : ganti semua kata `apple` (`regex` dasar) yang muncul dengan kata `mango` (`regex` dasar) di semua baris input dan cetak hasilnya ke `stdout`
- `perintah | sed -f path/to/script.sed` : jalankan skrip tertentu file dan cetak hasilnya ke `stdout`
- `perintah | sed -n '1p'` : cetak hanya baris pertama saja ke `stdout`

## Links
- More information: https://manned.org/sed.1posix
