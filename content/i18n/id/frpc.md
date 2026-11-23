# frpc

Hubungkan perangkat menuju jaringan proksi yang diatur oleh suatu peladen/server `frps`. Bagian dari `frp`.

## Usage
- `frpc` : jalankan layanan klien, menggunakan berkas konfigurasi bawaan/default (diasumsikan merupakan berkas `frps.ini` yang terletak pada direktori saat ini)
- `frpc -c jalan/menuju/frpc.toml` : jalankan layanan menggunakan berkas konfigurasi dengan format terbaru berbasis TOML (`frps.toml` daripada `frps.ini`) pada direktori saat ini
- `frpc -c jalan/menuju/berkas` : jalankan layanan menggunakan berkas konfigurasi tertentu
- `frpc verify -c jalan/menuju/berkas` : periksa apakah isi suatu berkas konfigurasi menggunakan format yang valid
- `frpc completion bash|fish|powershell|zsh` : tampilkan isi skrip shell yang perlu dijalankan untuk mengaktifkan fitur penyelesaian perintah otomatis (autocomplete) bagi Bash, fish, PowerShell, maupun Zsh
- `frpc -v` : tampilkan versi

## Links
- More information: https://github.com/fatedier/frp
