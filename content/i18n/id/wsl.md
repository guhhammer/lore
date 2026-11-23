# wsl

Atur modul Subsistem Windows untuk Linux (WSL).

## Usage
- `wsl perintah_syel` : jalankan suatu sesi syel Linux (di dalam distribusi default)
- `wsl -e perintah kumpulan_argumen_perintah` : jalankan suatu perintah Linux tanpa menggunakan suatu syel
- `wsl -d distribusi perintah_syel` : gunakan suatu distribusi (distro) tertentu
- `wsl -l` : tampilkan daftar distribusi yang tersedia
- `wsl --export distribusi jalan\menuju\berkas_distro.tar` : ekspor suatu distribusi ke dalam berkas `.tar`
- `wsl --import distribusi jalan\menuju\lokasi_pemasangan jalan/menuju/berkas_distro.tar` : impor suatu distribusi dari berkas `tar`
- `wsl --set-version distribusi versi` : ubah versi wsl yang digunakan untuk suatu distribusi
- `wsl --shutdown` : matikan Subsistem Windows untuk Linux

## Links
- More information: https://learn.microsoft.com/windows/wsl/reference
