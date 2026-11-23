# ps

Tampilkan informasi tentang proses-proses yang berlangsung.

## Usage
- `ps aux` : tampilkan daftar seluruh proses yang berlangsung
- `ps auxww` : tampilkan daftar seluruh proses yang berlangsung termasuk string perintah secara utuh
- `ps aux | grep [s]tring` : cari proses berdasarkan teks/string kriteria (penambahan kurung siku akan mencegah `grep` untuk mencocokkan dirinya sendiri)
- `ps -u $(id -u) -F` : tampilkan daftar seluruh proses dari pengguna saat ini dengan format tambahan ekstra
- `ps -u $(id -u) f` : tampilkan daftar seluruh proses dari pengguna saat ini dalam format pohon
- `ps -o ppid= -p pid` : mengambil induk PID dari sebuah proses
- `ps --sort size` : sortir proses berdasarkan konsumsi memori

## Links
- More information: https://manned.org/ps
