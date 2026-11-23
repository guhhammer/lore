# axel

Akselerator proses pengunduhan. Mendukung protokol HTTP, HTTPS, FTP, dan FTPS.

## Usage
- `axel url` : unduh isi suatu alamat URL menuju berkas
- `axel url -o jalan/menuju/berkas` : unduh dan tentukan nama akhir berkas
- `axel -n jumlah url` : tentukan jumlah koneksi saluran yang dipakai untuk mengunduh
- `axel -Sjumlah url` : tentukan jumlah sumber mirror yang dipakai untuk mencari dan mengunduh
- `axel -s kecepatan url` : batasi laju kecepatan pengunduhan (dalam bita per detik)
- `axel -4 url` : hanya gunakan protokol IPv4 saat berhubungan dengan host
- `axel -q -U "Mozilla/5.0" url` : batasi luaran program menuju `stdout` dan gunakan suatu nilai user-agent kustom saat mengunduh

## Links
- More information: https://manned.org/axel
