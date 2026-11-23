# aa-unconfined

Tampilkan daftar proses dengan port TCP/UDP terbuka yang tidak memiliki profil AppArmor yang dimuat.

## Usage
- `sudo aa-unconfined` : tampilkan daftar proses yang tidak dibatasi (unconfined) menggunakan perintah `ss` (default)
- `sudo aa-unconfined --with-netstat` : gunakan `netstat` alih-alih `ss` untuk mendeteksi soket jaringan yang terbuka
- `sudo aa-unconfined --paranoid` : tampilkan semua proses dari `/proc` dengan port TCP/UDP dan tanpa profil AppArmor (lebih terperinci)
- `aa-unconfined -h` : tampilkan bantuan

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-unconfined.8
