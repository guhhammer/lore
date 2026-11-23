# ssh-keyscan

Mengambil kunci publik SSH dari host jarak jauh (remote host).

## Usage
- `ssh-keyscan nama_host` : mengambil semua kunci publik SSH dari sebuah host jarak jauh (remote host)
- `ssh-keyscan -p port nama_host` : mengambil semua kunci publik SSH dari host jarak jauh (remote host) yang berjalan pada port tertentu
- `ssh-keyscan -t rsa,dsa,ecdsa,ed25519 nama_host` : mengambil tipe kunci publik SSH tertentu dari sebuah host jarak jauh (remote host)
- `ssh-keyscan -H nama_host >>~/.ssh/known_hosts` : perbarui berkas/fail `known_hosts` SSH secara manual dengan sidik jari (fingerprint) dari host yang diberikan

## Links
- More information: https://man.openbsd.org/ssh-keyscan
