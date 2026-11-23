# dd

Konversi dan salin file menuju media penyimpanan.

## Usage
- `dd if=jalan/menuju/berkas.iso of=/dev/perangkat_usb status=progress` : buat suatu perangkat USB bootable dari suatu berkas isohybrid (seperti `archlinux-xxx.iso`) dan tampilkan kemajuannya
- `dd bs=4194304 conv=fsync if=/dev/perangkat_asal of=/dev/perangkat_tujuan` : salin isi suatu perangkat penyimpanan menuju perangkat lain dengan ukuran blok 4 MiB dan lakukan flush write sebelum perintah berakhir
- `dd bs=100 count=1 if=/dev/urandom of=jalan/menuju/berkas_acak` : buat suatu berkas berisikan byte acak dari perangkat pengacak dalam kernel dengan jumlah ukuran berkas yang ditentukan
- `dd bs=1024 count=1000000 if=/dev/zero of=jalan/menuju/berkas_1GB` : ukur kinerja media penyimpanan dalam menulis data secara sekuensial
- `dd if=/dev/perangkat_penyimpanan of=jalan/menuju/berkas.img status=progress` : buat cadangan sistem, simpan ke dalam berkas IMG (dapat dipulihkan nanti dengan menukar `if` dan `of`), dan tampilkan kemajuannya

## Links
- More information: https://manned.org/dd.1p
