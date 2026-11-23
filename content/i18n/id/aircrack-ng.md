# aircrack-ng

Retas dan dapatkan kunci WEP dan WPA/WPA2 dari proses handshake dalam paket jaringan yang ditangkap. Bagian dari paket perangkat lunak jaringan Aircrack-ng.

## Usage
- `aircrack-ng -w jalan/menuju/wordlist.txt jalan/menuju/tangkapan_jaringan.cap` : retas dan dapatkan kunci dari berkas tangkapan jaringan dan berkas daftar kata sandi (wordlist)
- `aircrack-ng -p jumlah_thread -w jalan/menuju/wordlist.txt jalan/menuju/tangkapan_jaringan.cap` : retas dan dapatkan kunci menggunakan beberapa thread prosesor dari berkas tangkapan wordlist
- `aircrack-ng -w jalan/menuju/wordlist.txt -e extended_ssid jalan/menuju/tangkapan_jaringan.cap` : retas dan dapatkan kunci dari berkas tangkapan jaringan, wordlist, dan essid milik perangkat titik akses Wi-Fi
- `aircrack-ng -w jalan/menuju/wordlist.txt --bssid mac jalan/menuju/tangkapan_jaringan.cap` : retas dan dapatkan kunci dari berkas tangkapan jaringan, wordlist, dan alamat MAC milik perangkat titik akses Wi-Fi

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=aircrack-ng
