# aide

Advanced Intrusion Detection Environment untuk memvalidasi integritas file.

## Usage
- `sudo aide -i` : lakukan inisialisasi database
- `sudo aide -C` : periksa database untuk mencari inkonsistensi
- `sudo aide -E` : bandingkan dua database berdasarkan definisi dalam file konfigurasi
- `sudo aide -u` : periksa dan perbarui database secara non-interaktif
- `sudo aide -c path/ke/config_file` : tentukan file konfigurasi untuk menimpa `aide.conf` bawaan
- `sudo aide -l regex` : gunakan `regex` untuk membatasi AIDE pada string tertentu
- `sudo aide -r reporterurl` : kirimkan hasil reporter ke sebuah URL

## Links
- More information: https://manned.org/aide
