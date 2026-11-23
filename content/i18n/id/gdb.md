# gdb

GNU Debugger, alat pengawakutu program komputer.

## Usage
- `gdb jalan/menuju/berkas_exe` : jalankan pengawakutu pada sebuah berkas program yang dapat dieksekusi
- `gdb -p berkas_exe` : tambahkan suatu proses untuk diawasi oleh `gdb`
- `gdb -c jalan/menuju/berkas_core jalan/menuju/berkas_exe` : jalankan pengawakutu dengan berkas core (rekaman memnori dan diagnostik kesalahan program) yang ditentukan
- `gdb -ex "perintah" jalan/menuju/berkas_exe` : kirim perintah menuju pengawakutu pada saat dijalankan
- `gdb --args jalan/menuju/berkas_exe argumen1 argumen2 ...` : jalankan `gdb` dan teruskan argumen-argumen perintah ke dalam program yang diawasi
- `gdb -c jalan/menuju/berkas_core jalan/menuju/berkas_exe -iex 'set debuginfod enabled on' -iex 'set pagination off' -ex bt` : lewati `debuginfod` dan fitur navigasi, dan kemudian segera cetak jejak balik eksekusi program jika terdapat kesalahan

## Links
- More information: https://sourceware.org/gdb/current/onlinedocs/gdb#Invocation
