# warp-cli

Program command-line resmi untuk layanan Cloudflare WARP. WARP adalah sebuah layanan jaringan privat virtual (VPN) yang mengenkripsi lalu lintas jaringan demi meningkatkan privasi, keamanan, dan kecepatan.

## Usage
- `warp-cli registration new` : daftarkan perangkat ini ke dalam jaringan WARP (harus dijalankan pada pertama kali)
- `warp-cli connect` : hubungkan perangkat ini ke dalam jaringan WARP
- `warp-cli disconnect` : putuskan perangkat ini dari jaringan WARP
- `warp-cli status` : tampilkan status koneksi WARP saat ini
- `warp-cli mode warp|doh|warp+doh|dot|warp+dot|proxy|tunnel_only` : pindah mode operasi koneksi layanan WARP

## Links
- More information: https://developers.cloudflare.com/warp-client/
