# aa-audit

Atur profil keamanan AppArmor ke mode audit.

## Usage
- `sudo aa-audit profile_name` : atur profil ke mode audit
- `sudo aa-audit profile1 profile2 ...` : atur beberapa profil ke mode audit
- `sudo aa-audit -d /path/ke/profil profile_name` : atur profil ke mode audit dari direktori tertentu
- `sudo aa-audit --force profile_name` : paksa mode audit meskipun sudah diterapkan
- `sudo aa-audit --no-reload profile_name` : atur profil ke mode audit tanpa memuat ulang (reload)
- `sudo aa-audit -r profile_name` : hapus mode audit untuk sebuah profil
- `aa-audit -h` : tampilkan bantuan

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-audit.8
