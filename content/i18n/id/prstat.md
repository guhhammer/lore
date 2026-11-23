# prstat

Laporkan statistik proses aktif.

## Usage
- `prstat` : periksa semua proses dan laporan statistik yang diurutkan berdasarkan tingkat penggunaan CPU
- `prstat -s rss` : periksa semua proses dan laporan statistik yang disortir berdasarkan penggunaan memori
- `prstat -t` : laporkan ringkasan total penggunaan untuk tiap user
- `prstat -m` : laporkan informasi pengukuran proses microstate
- `prstat -c -n 5 -s cpu 1` : cetak daftar penggunaan CPU 5 proses teratas tiap 1 detik

## Links
- More information: https://www.unix.com/man-page/sunos/1m/prstat
