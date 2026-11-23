# arjun

Temukan parameter HTTP yang tersedia untuk sistem aplikasi web.

## Usage
- `arjun -u https://example.com/page.php` : pindai suatu URL untuk mendapatkan parameter GET yang tersedia
- `arjun -u https://example.com/api -m POST` : pindai menggunakan metode permintaan POST
- `arjun -u https://example.com -o jalan/menuju/luaran.json` : simpan hasil pemindaian ke dalam suatu berkas JSON
- `arjun -u https://example.com -w jalan/menuju/daftar_kata.txt` : gunakan berkas daftar kata (wordlist) tertentu untuk memindai
- `arjun -u https://example.com -d 2` : perbesar jumlah detik waktu tunggu antar permintaan untuk menghindari pembatasan laju permintaan

## Links
- More information: https://github.com/s0md3v/Arjun/wiki/Usage
