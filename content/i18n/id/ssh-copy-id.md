# ssh-copy-id

Pasang kunci publik Anda di berkas/fail authorized_keys pada mesin jarak jauh (remote host).

## Usage
- `ssh-copy-id nama_pengguna@host_jarak_jauh` : salin kunci Anda ke mesin jarak jauh (remote host)
- `ssh-copy-id -i lokasi/ke/kunci_publik nama_pengguna@host_jarak_jauh` : salin kunci publik yang diberikan ke mesin jarak jauh (remote host)
- `ssh-copy-id -i lokasi/ke/kunci_publik -p port nama_pengguna@host_jarak_jauh` : salin kunci publik yang diberikan ke mesin jarak jauh (remote host) dengan port spesifik

## Links
- More information: https://manned.org/ssh-copy-id
