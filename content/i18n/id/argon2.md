# argon2

Hitung kode hash menggunakan algoritma kriptografi Argon2.

## Usage
- `echo "kata_sandi" | argon2 "kata_garam"` : hitung kode hash dari suatu kata kunci (password) dengan suatu kata garam (salt) menggunakan parameter kriptografi bawaan
- `echo "kata_sandi" | argon2 "kata_garam" -d|i|id` : hitung kode hash dengan algoritma tertentu
- `echo "kata_sandi" | argon2 "kata_garam" -e` : jangan tampilkan informasi tambahan selain hasil kode hash
- `echo "kata_sandi" | argon2 "kata_garam" -t 5 -m 20 -p 7` : hitung kode hash dengan konfigurasi waktu, pemanfaatan memori (RAM), dan paralelisme pada pemrosesan kriptografi secara tertentu

## Links
- More information: https://github.com/P-H-C/phc-winner-argon2#command-line-utility
