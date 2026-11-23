# aa-cleanprof

Bersihkan profil keamanan AppArmor dengan menghapus aturan yang tidak digunakan.

## Usage
- `sudo aa-cleanprof profile_name` : bersihkan profil untuk menghapus aturan yang tidak digunakan
- `sudo aa-cleanprof profile1 profile2 ...` : bersihkan beberapa profil sekaligus
- `sudo aa-cleanprof -d /path/ke/profil profile_name` : tentukan direktori yang berisi profil
- `sudo aa-cleanprof -s profile_name` : jalankan secara senyap tanpa prompt
- `sudo aa-cleanprof --no-reload profile_name` : cegah muat ulang profil setelah pembersihan
- `aa-cleanprof -h` : tampilkan bantuan

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-cleanprof.8
