# ar

Buat, olah, dan ekstrak berkas dalam format arsip Unix. Biasanya dimanfaatkan untuk pustaka statis (`.a`) dan paket piranti lunak Debian (`.deb`).

## Usage
- `ar x jalan/menuju/berkas.a` : extrak seluruh berkas dalam suatu arsip
- `ar t jalan/menuju/berkas.ar` : lihat daftar isi dari suatu arsip
- `ar r jalan/menuju/berkas.deb jalan/menuju/debian-binary jalan/menuju/control.tar.gz jalan/menuju/data.tar.xz ...` : gantikan (replace) atau tambahkan suatu berkas ke dalam arsip
- `ar s jalan/menuju/berkas.a` : masukkan suatu berkas objek (setara dengan penggunaan `ranlib`)
- `ar rs jalan/menuju/berkas.a jalan/menuju/berkas1.o jalan/menuju/berkas2.o ...` : buat suatu arsip berisikan kumpulan berkas tertentu, dan suatu berkas daftar indeks para objek

## Links
- More information: https://manned.org/ar
