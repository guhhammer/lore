# aa-genprof

Buat profil keamanan AppArmor dengan memantau perilaku program.

## Usage
- `sudo aa-genprof program_path` : mulai buat profil untuk sebuah program
- `sudo aa-genprof -d /path/ke/profil program_path` : tentukan direktori kustom untuk profil
- `sudo aa-genprof -f /path/ke/file_log program_path` : tentukan file log kustom untuk profiling
- `aa-genprof -h` : tampilkan bantuan

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-genprof.8
