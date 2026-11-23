# apparmor_parser

Muat, kompilasi, dan kelola profil keamanan AppArmor.

## Usage
- `sudo apparmor_parser -a nama_profil` : muat profil ke dalam kernel
- `sudo apparmor_parser -r nama_profil` : ganti profil yang sudah ada
- `sudo apparmor_parser -R nama_profil` : hapus profil dari kernel
- `sudo apparmor_parser -C -r jalan/menuju/profil` : muat profil dalam mode komplain (mencatat pelanggaran tetapi tidak memblokir)
- `apparmor_parser -p -o jalan/menuju/output.cache -Q jalan/menuju/profil` : lakukan praproses pada profil (selesaikan include) dan tulis cache biner ke file
- `apparmor_parser -p -S -Q jalan/menuju/profil` : lakukan praproses dan cetak profil biner ke `stdout` tanpa memuatnya
- `sudo apparmor_parser -r -T jalan/menuju/profil` : ganti profil sambil mengabaikan pembacaan cache
- `sudo apparmor_parser -r -W -L /jalan/menuju/cache jalan/menuju/profil` : ganti profil, bangun ulang cache, dan tulis ke direktori kustom

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_apparmor_parser.8
