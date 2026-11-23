# aria2c

Utilitas unduhan cepat. Mendukung HTTP(S), FTP, SFTP, BitTorrent, dan Metalink.

## Usage
- `aria2c "url"` : unduh URI ke suatu berkas
- `aria2c -o jalan/menuju/berkas "url"` : unduh berkas yang ditunjuk oleh URI yang ditentukan dengan nama keluaran yang ditentukan
- `aria2c -Z "url1" "url2" ...` : unduh beberapa berkas (berbeda) secara paralel
- `aria2c --checksum sha-256=hash "url1" "url2" ...` : unduh berkas yang sama dari kumpulan sumber mirror dan lakukan verifikasi atas berkas yang diunduh
- `aria2c -i jalan/menuju/berkas -j jumlah_unduhan` : unduh URI yang tercantum dalam suatu berkas dengan unduhan paralel terbatas
- `aria2c -s jumlah_koneksi "url"` : unduh dengan berbagai koneksi
- `aria2c --ftp-user nama_pengguna --ftp-passwd kata_sandi "url"` : unduh berkas dari peladen FTP dengan username pengguna dan kata sandi
- `aria2c --max-download-limit kecepatan "url"` : batasi kecepatan unduh dalam bytes/detik

## Links
- More information: https://aria2.github.io/manual/en/html/aria2c.html
