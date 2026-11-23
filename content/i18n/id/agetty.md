# agetty

Alternatif `getty`: Buka sebuah port `tty`, minta nama login, dan jalankan perintah `/bin/login`. Biasanya dijalankan oleh `init`. Catatan: Baud rate adalah kecepatan transfer data antara terminal dan perangkat melalui koneksi serial.

## Usage
- `agetty tty 115200` : hubungkan `stdin` ke sebuah port (relatif terhadap `/dev`) dan secara opsional tentukan baud rate (secara bawaan 9600)
- `agetty -t timeout_in_seconds -` : asumsikan `stdin` sudah terhubung ke sebuah `tty` dan atur timeout untuk login
- `agetty -8 - term_var` : asumsikan `tty` adalah 8-bit, menimpa variabel environment `$TERM` yang diatur oleh `init`
- `agetty -n -l login_program tty` : lewati proses login (tanpa login) dan jalankan program login lain sebagai root, alih-alih `/bin/login`
- `agetty -i -` : jangan tampilkan file pra-login (issue) (`/etc/issue` secara bawaan) sebelum menuliskan prompt login
- `agetty -r /path/ke/root_directory -H fake_host -` : ubah direktori root dan tulis host palsu tertentu ke dalam file `utmp`

## Links
- More information: https://manned.org/agetty
