# htpasswd

Buat dan atur isi berkas-berkas htpasswd untuk melindungi kumpulan direktori yang diakses oleh peladen web menggunakan metode autentikasi sederhana/basic.

## Usage
- `htpasswd -c jalan/menuju/berkas nama_pengguna` : buat/timpa isi berkas htpasswd
- `htpasswd jalan/menuju/berkas nama_pengguna` : tambahkan atau mutakhirkan konfigurasi bagi suatu akun pengguna (dalam mengakses layanan web) ke dalam berkas htpasswd
- `htpasswd -b jalan/menuju/berkas nama_pengguna kata_sandi` : tambahkan suatu akun pengguna kepada berkas htpasswd dalam mode batch, dengan melewati proses pemasukkan kata sandi secara interaktif (untuk penggunaan dalam skrip syel)
- `htpasswd -D jalan/menuju/berkas nama_pengguna` : hapus konfigurasi akses suatu akun pengguna dari isi suatu berkas htpasswd
- `htpasswd -v jalan/menuju/berkas nama_pengguna` : lakukan proses verifikasi kata sandi bagi suatu akun pengguna
- `htpasswd -nbm nama_pengguna kata_sandi` : tampilkan suatu string berisikan username (akun pengguna, dalam plain text) dan kata sandi (dalam hash MD5)

## Links
- More information: https://httpd.apache.org/docs/current/programs/htpasswd.html
