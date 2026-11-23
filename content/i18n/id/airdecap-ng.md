# airdecap-ng

Dekripsi file tangkapan jaringan terenkripsi dengan kunci sandi WEP, WPA, atau WPA2. Bagian dari paket perangkat lunak jaringan Aircrack-ng.

## Usage
- `airdecap-ng -b alamat_mac jalan/menuju/tangkapan_jaringan.cap` : buang informasi header jaringan wireless/nirkabel dari file tangkapan jaringan (bebas enkripsi WEP/WPA/WPA2), dan gunakan alamat MAC titik akses Wi-Fi untuk menyaringnya
- `airdecap-ng -w kunci_heksadesimal jalan/menuju/tangkapan_jaringan.cap` : buka enkripsi WEP dari file tangkapan jaringan menggunakan kunci WEP dalam format heksadesimal
- `airdecap-ng -e extended_ssid -p kata_sandi jalan/menuju/tangkapan_jaringan.cap` : buka enkripsi WPA/WPA2 dari file tangkapan jaringan menggunakan essid titik akses Wi-Fi dan kata sandi (password)
- `airdecap-ng -l -e extended_ssid -p kata_sandi jalan/menuju/tangkapan_jaringan.cap` : buka enkripsi WPA/WPA2 dari file tangkapan jaringan menggunakan essid dan kata sandi (password), tanpa menghilangkan informasi header jaringan
- `airdecap-ng -b alamat_mac -e extended_ssid -p kata_sandi jalan/menuju/tangkapan_jaringan.cap` : buka enkripsi WPA/WPA2 dari file tangkapan jaringan menggunakan essid dan kata sandi (password), dan saring menurut alamat MAC titik akses Wi-Fi

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=airdecap-ng
