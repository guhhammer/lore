# acme.sh

Sebuah shell script yang mengimplementasikan ACME client protocol (pembuat sertifikat HTTPS), alternatif dari `certbot`.

## Usage
- `acme.sh --issue -d example.com -w /jalan/menuju/webroot` : terbitkan sertifikat baru dan pasang pada webroot secara langsung
- `acme.sh --issue --standalone -d example.com -d www.example.com` : terbitkan sertifikat untuk domain majemuk, menggunakan mode verifikasi mandiri (standalone) pada port 80
- `acme.sh --issue --alpn -d example.com` : terbitkan sertifikat menggunakan mode verifikasi mandiri berbasis TLS pada port 443
- `acme.sh --issue --nginx -d example.com` : terbitkan sertifikat dengan konfigurasi server `nginx` untuk memasangnya
- `acme.sh --issue --apache -d example.com` : terbitkan sertifikat dengan konfigurasi server Apache untuk memasangnya
- `acme.sh --issue --dns api_dns -d *.example.com` : terbitkan sertifikat wildcard (\*) menggunakan mode verifikasi DNS via API secara otomatis
- `acme.sh -i -d example.com --key-file /jalan/menuju/example.com.key --fullchain-file /jalan/menuju/example.com.cer --reloadcmd "systemctl force-reload nginx"` : pasang sertifikat ke dalam direktori tertentu (dapat berguna untuk proses pemutakhiran otomatis)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
