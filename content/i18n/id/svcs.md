# svcs

Ambil atau atur sumber daya dari proses, tugas dan projek yang berjalan.

## Usage
- `svcs` : daftar semua servis yang berjalan
- `svcs -vx` : daftar servis-servis yang tidak berjalan
- `svcs apache` : daftar informasi tentang sebuah servis
- `svcs -L apache` : tampilkan lokasi dari berkas catatan servis
- `tail $(svcs -L apache)` : tampilkan isi akhir suatu berkas log servis

## Links
- More information: https://www.unix.com/man-page/sunos/1/svcs
