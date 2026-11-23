# aa-logprof

Perbarui profil keamanan AppArmor secara interaktif berdasarkan pelanggaran yang tercatat.

## Usage
- `sudo aa-logprof` : tinjau dan perbarui profil secara interaktif berdasarkan log sistem
- `sudo aa-logprof -d /path/ke/profil` : gunakan direktori tertentu untuk profil AppArmor
- `sudo aa-logprof -f /path/ke/file_log` : gunakan file log tertentu alih-alih default
- `sudo aa-logprof -m "log_marker_text"` : abaikan semua entri log sebelum tanda yang ditentukan
- `aa-logprof -h` : tampilkan bantuan

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-logprof.8
