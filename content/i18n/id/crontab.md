# crontab

Jadwalkan tugas-tugas cron untuk dijalankan pada interval waktu bagi pengguna saat ini.

## Usage
- `crontab -e` : sunting (edit) berkas crontab untuk pengguna saat ini
- `sudo crontab -e -u pengguna` : sunting berkas crontab untuk pengguna (user) tertentu
- `crontab jalan/menuju/berkas` : ganti seluruh isi crontab saat ini dengan isi dari berkas tertentu
- `crontab -l` : tampilkan (list) daftar tugas cron untuk pengguna saat ini
- `crontab -r` : hapuskan (remove) seluruh tugas cron untuk pengguna saat ini
- `0 10 * * * perintah_untuk_dijalankan` : contoh tugas cron yang akan dijalankan pukul 10:00 setiap hari (* berarti nilai apapun)
- `*/10 * * * * perintah_untuk_dijalankan` : contoh tugas cron yang akan dijalankan setiap 10 menit
- `30 2 * * Fri /jalan/menuju/naskah.sh` : contoh tugas cron yang akan menjalankan suatu naskah pada pukul 02:30 setiap Jumat

## Links
- More information: https://manned.org/crontab
