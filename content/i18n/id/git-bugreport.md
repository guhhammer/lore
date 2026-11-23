# git bugreport

Tangkap dan simpan informasi sistem dan pengguna Git dalam berkas teks untuk kepentingan melaporkan dan menyelesaikan masalah/bug internal dalam Git.

## Usage
- `git bugreport` : buat berkas laporan masalah/bug baru pada direktori saat ini
- `git bugreport -o jalan/menuju/direktori` : buat berkas laporan pada direktori tertentu, dan buat direktori tersebut jika belum
- `git bugreport -s %m%d%y` : buat berkas laporan baru, dengan nama berkas diakhiri dengan tanggal pelaporan menurut format `strftime`

## Links
- More information: https://git-scm.com/docs/git-bugreport
