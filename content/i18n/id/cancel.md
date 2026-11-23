# cancel

Batalkan penugasan pencetakan dokumen terhadap mesin-mesin pencetak (printer).

## Usage
- `cancel` : batalkan pekerjaan yang sedang dikerjakan ke dalam mesin pencetak yang diatur sebagai tujuan bawaan (atur mesin tujuan bawaan dengan perintah `lpoptions -d pencetak`)
- `cancel -u nama_pengguna` : batalkan pekerjaan yang sedang dikerjakan ke dalam mesin pencetak yang diatur sebagai tujuan bawaan menurut pengaturan suatu pengguna (user)
- `cancel pencetak` : batalkan pekerjaan yang sedang dikerjakan ke dalam suatu mesin pencetak
- `cancel pencetak-nomor_induk_pekerjaan` : batalkan suatu pekerjaan yang sedang dikerjakan ke dalam suatu mesin pencetak
- `cancel -a` : batalkan semua pekerjaan yang sedang dan akan dikerjakan oleh mesin pencetak apapun
- `cancel -a pencetak` : batalkan semua pekerjaan yang sedang dan akan dikerjakan oleh suatu mesin pencetak
- `cancel -h peladen -x` : batalkan pekerjaan yang sedang ditangani oleh suatu peladen (server) layanan pencetak, kemudian hapus (x) berkas-berkas data pendukung pekerjaan

## Links
- More information: https://openprinting.github.io/cups/doc/man-cancel.html
