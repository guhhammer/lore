# where

Tampilkan lokasi kumpulan berkas yang memenuhi kriteria suatu pola pencarian alamat berkas. Lokasi pencarian default diatur sebagai direktori kerja saat ini beserta kumpulan alamat yang diatur dalam variabel lingkungan `%PATH%`.

## Usage
- `where pola_pencarian` : tampilkan daftar alamat berkas yang memenuhi pola pencarian alamat
- `where /T pola_pencarian` : tampilkan hasil pencarian termasuk ukuran dan tanggal berkas
- `where /R jalan\menuju\direktori pola_pencarian` : lakukan hasil pencarian terhadap suatu direktori secara rekursif
- `where /Q pola_pencarian` : hanya kembalikan kode galat dari hasil pencarian tanpa menampilkan daftar alamat
