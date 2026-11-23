# a2query

Dapatkan konfigurasi yang dipakai saat ini (secara runtime) dari piranti peladen Apache dalam sistem operasi berbasis Debian.

## Usage
- `sudo a2query -m` : tampilkan daftar modul Apache yang sedang aktif
- `sudo a2query -m nama_modul` : cek apakah suatu modul Apache sedang aktif
- `sudo a2query -s` : tampilkan daftar host maya (virtual hosts) yang sedang aktif
- `sudo a2query -M` : tampilkan jenis modul Multi Processing Module yang sedang aktif
- `sudo a2query -v` : tampilkan versi piranti peladen Apache

## Links
- More information: https://manned.org/a2query
