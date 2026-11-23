# curl

Mentransfer data dari atau menuju suatu server. Mendukung sebagian besar protokol, termasuk HTTP, FTP, dan POP3.

## Usage
- `curl https://example.com` : buat suatu permintaan HTTP GET dan tampilkan respons menuju `stdout`
- `curl -L -D - https://example.com` : buat suatu permintaan HTTP GET, ikuti (foLlow) seluruh permintaan redirect `3xx`, dan tampilkan (Dump) seluruh data header beserta isi respons menuju `stdout`
- `curl -O http://example.com/nama_berkas.zip` : unduh dan simpan suatu berkas dengan nama berkas yang ditentukan oleh URL
- `curl -X POST -d 'name=bob' http://example.com/form` : kirim suatu data dengan format form-encoded (permintaan POST dengan format data `application/x-www-form-urlencoded`). Gunakan `--data @file_name` atau `--data @'-'` untuk membaca dari `stdin`
- `curl -k -x http://127.0.0.1:8080 -H 'Authorization: Bearer token' -X GET|PUT|POST|DELETE|PATCH|... https://example.com` : kirim sebuah permintaan dengan suatu informasi header ekstra, menggunakan metode HTTP kustom yang dikirimkan menggunakan suatu proxi (seperti BurpSuite), mengabaikan peringatan sertifikat TLS yang ditandatangani secara mandiri (self-signed)
- `curl -d '{"name":"bob"}' -H 'Content-Type: application/json' http://example.com/pengguna/1234` : kirim data dalam format JSON, dengan menetapkan isi HTTP Header Content-Type
- `curl -E sertifikat_klien.pem --key kunci.pem -k https://example.com` : gunakan suatu sertifikat klien dan berkas kunci dalam mengenkripsi permintaan HTTP, dengan mengabaikan proses validasi sertifikat TLS
- `curl -v --resolve example.com:80:127.0.0.1 http://example.com` : ubah nama host menjadi alamat IP khusus, dengan keluaran verbose (mirip dengan menyunting berkas `/etc/hosts` untuk resolusi DNS khusus)

## Links
- More information: https://curl.se/docs/manpage.html
