# export

Ekspor variabel menuju anak-anak proses syel sistem operasi.

## Usage
- `export VARIABEL=nilai` : setel nilai suatu variabel lingkungan syel (environment variable)
- `export -n VARIABEL` : hapus nilai variabel lingkungan
- `export -f NAMA_FUNGSI` : ekspor suatu fungsi perintah (function) menuju anak-anak proses syel
- `export PATH=$PATH:path/to/append` : tambahkan alamat direktori baru menuju variabel lingkungan `$PATH`
- `export -p` : tampilkan daftar variabel yang telah diekspor dalam bentuk kode perintah syel

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-export
