# chpass

Tambahkan atau ubah informasi dalam pangkalan data pengguna sistem operasi, termasuk syel masuk dan kata sandi. Catatan: Kata sandi pengguna tidak dapat diubah pada sistem Open Directory, gunakan `passwd` sebagai gantinya.

## Usage
- `su -c chpass` : tentukan syel masuk secara spesifik untuk pengguna saat ini secara interaktif
- `chpass -s jalan/menuju/syel` : tentukan syel masuk secara spesifik untuk pengguna saat ini
- `chpass -s jalan/menuju/syel nama_pengguna` : tentukan syel masuk untuk suatu pengguna
- `chpass -l lokasi -s jalan/menuju/syel nama_pengguna` : ubah rekam pengguna dalam node direktori pada lokasi yang ditentukan
- `chpass -u username_untuk_autentikasi -s path/to/shell nama_pengguna` : gunakan suatu nama pengguna (username) saat melakukan proses masuk menuju node direktori yang memiliki informasi pengguna

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=chpass
