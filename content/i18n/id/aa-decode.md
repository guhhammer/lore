# aa-decode

Dekode log audit AppArmor menjadi format yang dapat dibaca manusia.

## Usage
- `aa-decode hexstring` : dekode string heksadesimal
- `sudo aa-decode logfile` : dekode file log
- `sudo aa-decode - <logfile` : dekode log dari `stdin` (contoh: file yang dialihkan/ redirected)
- `aa-decode -h` : tampilkan bantuan

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-decode.8
