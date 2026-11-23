# truss

Alat pemecah masalah untuk menelusuri panggilan sistem. Perintah setara strace untuk SunOS.

## Usage
- `truss -f program` : telusuri sebuah program dengan mengeksekusinya dan mengawasi semua proses turunan
- `truss -p pid` : mulai menelusuri sebuah proses tertentu berdasarkan PID-nya
- `truss -a -e program` : mulai menelusuri sebuah program dengan mengeksekusinya, tampilkan argumen-argumen dan variabel-variabel lingkungan
- `truss -c -p pid` : menghitung waktu, panggilan, dan error untuk setiap panggilan sistem dan laporkan sebuah ringkasan saat keluar program
- `truss -p pid -t nama_panggilan_sistem` : telusuri proses berdasarkan keluaran dari panggilan sistem

## Links
- More information: https://www.unix.com/man-page/sunos/1/truss
