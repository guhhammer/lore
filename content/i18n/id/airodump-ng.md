# airodump-ng

Tangkap para paket dan tampilkan informasi mengenai jaringan nirkabel/wireless. Bagian dari paket perangkat lunak jaringan Aircrack-ng.

## Usage
- `sudo airodump-ng interface` : tangkap para paket dan tampilkan daftar jaringan nirkabel dalam frekuensi 2.4GHz
- `sudo airodump-ng interface --band a` : tangkap para paket dan tampilkan daftar jaringan nirkabel dalam frekuensi 5GHz
- `sudo airodump-ng interface --band abg` : tangkap para paket dan tampilkan daftar jaringan nirkabel, baik dalam frekuensi 2.4GHz maupun 5GHz
- `sudo airodump-ng --channel channel --write jalan/menuju/berkas --bssid alamat_mac interface` : tangkap para paket dan tampilkan informasi jaringan nirkabel berdasarkan alamat MAC dan kanal jaringan, kemudian simpan hasil ke dalam suatu file

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=airodump-ng
