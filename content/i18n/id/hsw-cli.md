# hsw-cli

Alat baris perintah untuk mengakses dompet digital Handshake melalui koneksi REST API.

## Usage
- `hsw-cli unlock kata_sandi timeout` : buka akses terhadap suatu dompet digital (nilai timeout dalam hitungan detik)
- `hsw-cli lock` : kunci dompet saat ini
- `hsw-cli get` : tampilkan informasi terhadap dompet saat ini
- `hsw-cli balance` : tampilkan saldo dompet saat ini
- `hsw-cli history` : tampilkan riwayat transaksi yang menggunakan dompet saat ini
- `hsw-cli send alamat_tujuan 1.05` : kirim suatu transaksi dengan suatu nominal koin menuju suatu alamat dompet tujuan
- `hsw-cli pending` : tampilkan daftar transaksi yang berstatus tertunda (pending) yang melibatkan dompet ini
- `hsw-cli tx hash_transaksi` : tampilkan informasi rincian suatu transaksi

## Links
- More information: https://github.com/handshake-org/hs-client
