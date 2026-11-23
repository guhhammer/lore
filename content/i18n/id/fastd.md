# fastd

Program layanan daemon untuk jaringan priat virtual (VPN). Bekerja baik dalam lapisan jaringan Layer 2 atau Layer 3, mendukung berbagai metode enkripsi, dan dipakai oleh Freifunk.

## Usage
- `fastd -c jalan/menuju/fastd.conf` : jalankan `fastd` dengan konfigurasi yang diatur dalam suatu berkas
- `fastd -m tap -M 1400 -c jalan/menuju/fastd.conf` : jalankan suatu layanan VPN Layer 3 dengan MTU sebesar 1400, dengan konfigurasi lainnya yang diatur dalam suatu berkas
- `fastd --verify-config -c jalan/menuju/fastd.conf` : lakukan validasi terhadap suatu berkas konfigurasi
- `fastd --generate-key` : buat sebuah pasangan kunci untuk mengakses layanan VPN
- `fastd --show-key -c jalan/menuju/fastd.conf` : tampilkan kunci publik terhadap kunci privat yang diatur dalam berkas konfigurasi
- `fastd -v` : tampilkan versi

## Links
- More information: https://fastd.readthedocs.io/en/stable/
