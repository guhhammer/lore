# apachectl

Kontrol server HTTP Apache.

## Usage
- `sudo apachectl start` : jalankan peladen
- `sudo apachectl restart` : jalankan ulang peladen
- `sudo apachectl stop` : hentikan peladen
- `apachectl configtest` : lakukan uji validitas file konfigurasi
- `apachectl status` : periksa status peladen (membutuhkan peramban lynx)
- `sudo apachectl graceful` : lakukan muat ulang konfigurasi tanpa memutus koneksi
- `apachectl -S` : tampilkan konfigurasi lengkap Apache (tidak selalu didukung)
- `apachectl -h` : tampilkan bantuan

## Links
- More information: https://manned.org/apachectl
