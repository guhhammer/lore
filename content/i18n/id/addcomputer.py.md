# addcomputer.py

Tambahkan akun komputer menuju suatu domain. Bagian dari kelompok peranti Impacket.

## Usage
- `addcomputer.py -computer-name NAMA_KOMPUTER$ -computer-pass kata_sandi_komputer domain/nama_pengguna:kata_sandi` : tambahkan suatu komputer dengan nama dan kata sandi
- `addcomputer.py -no-add -computer-name NAMA_KOMPUTER$ -computer-pass kata_sandi_komputer domain/nama_pengguna:kata_sandi` : hanya atur kata sandi untuk perangkat komputer saat ini
- `addcomputer.py -delete -computer-name NAMA_KOMPUTER$ domain/nama_pengguna:kata_sandi` : hapus suatu akun perangkat komputer
- `addcomputer.py -k -no-pass domain/nama_pengguna@nama_host` : tambahkan perangkat komputer melalui protokol autentikasi Kerberos
- `addcomputer.py -method LDAPS -port 636 domain/nama_pengguna:kata_sandi` : tambahkan perangkat komputer melalui protokol LDAPS (port 636) daripada SAMR (port 445)
- `addcomputer.py -dc-host nama_host domain/nama_pengguna:kata_sandi` : tentukan nama pengontrol domain (DC) yang akan dipakai, jika terdapat beberapa DC yang terhubung

## Links
- More information: https://github.com/fortra/impacket
