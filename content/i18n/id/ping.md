# ping

Kirim kumpulan paket ICMP ECHO_REQUEST (sebagai pesan "ping") ke host dalam jaringan.

## Usage
- `ping host` : ping suatu host
- `ping -c jumlah host` : ping suatu host dengan jumlah pengulangan yang ditetapkan
- `ping -i detik host` : ping suatu host, dengan menentukan interval dalam sekian detik di antara permintaan (asalnya 1 detik)
- `ping -n host` : ping suatu host tanpa mencoba untuk melihat alamat dari nama-nama simbolis
- `ping -a host` : ping suatu host dan membunyikan bel saat paket diterima (jika terminal anda mendukungnya)
- `ping -O host` : tampilkan juga pesan jika tidak ada respon yang diterima
- `ping -c jumlah -W detik -w detik host` : ping suatu host dengan jumlah pengulangan yang ditetapkan, batas timeout (`-W`) untuk setiap balasan, serta batas waktu (`-w`) eksekusi bagi keseluruhan proses ping berjalan

## Links
- More information: https://manned.org/ping
