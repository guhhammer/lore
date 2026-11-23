# frps

Buat suatu peladen pelayan jaringan proksi terbalik (reverse proksi). Bagian dari `frp`.

## Usage
- `frps` : jalankan layanan peladen, menggunakan berkas konfigurasi bawaan/default (diasumsikan merupakan berkas `frps.ini` yang terletak pada direktori saat ini)
- `frps -c jalan/menuju/frps.toml` : jalankan layanan menggunakan berkas konfigurasi dengan format terbaru berbasis TOML (`frps.toml` daripada `frps.ini`) pada direktori saat ini
- `frps -c jalan/menuju/berkas` : jalankan layanan menggunakan berkas konfigurasi tertentu
- `frps verify -c jalan/menuju/berkas` : periksa apakah isi suatu berkas konfigurasi menggunakan format yang valid
- `frps completion bash|fish|powershell|zsh` : tampilkan isi skrip shell yang perlu dijalankan untuk mengaktifkan fitur penyelesaian perintah otomatis (autocomplete) bagi Bash, fish, PowerShell, maupun Zsh
- `frps -v` : tampilkan versi

## Links
- More information: https://github.com/fatedier/frp
