# caddy

Sebuah peladen web bersumber terbuka yang siap pakai untuk perusahaan, dengan HTTPS otomatis, dan ditulis dalam bahasa Go.

## Usage
- `caddy run` : jalankan Caddy sebagai proses latar depan (foreground)
- `caddy run --config jalan/menuju/Caddyfile` : jalankan Caddy dengan konfigurasi Caddyfile tertentu
- `caddy start` : jalankan Caddy dalam latar belakang (background)
- `caddy stop` : hentikan suatu proses Caddy yang berjalan di latar belakang
- `caddy file-server --listen :8000 --browse` : jalankan suatu peladen berkas sederhana dengan port spesifik, dengan fitur antarmuka penjelajah berkas
- `caddy reverse-proxy --from :80 --to localhost:8000` : jalankan suatu peladen proksi terbalik (reverse proxy)

## Links
- More information: https://caddyserver.com/docs/command-line
