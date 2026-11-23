# abrt-cli

Automatic Bug Reporting Tool untuk sistem berbasis Fedora. Digunakan untuk mendeteksi, menganalisis, dan melaporkan crash aplikasi.

## Usage
- `abrt-cli list` : tampilkan daftar masalah yang terdeteksi
- `abrt-cli info problem_id` : tampilkan detail dari masalah tertentu
- `abrt-cli remove problem_id` : hapus laporan crash
- `abrt-cli report problem_id` : laporkan masalah ke bug tracker yang terkonfigurasi (misalnya Bugzilla)
- `abrt-watch-log -F error_string /var/log/myapp.log notify-send "Crash detected"` : pantau file log dan picu program saat kecocokan ditemukan
- `abrt-cli report -a problem_id` : buat laporan untuk debugging secara manual

## Links
- More information: https://abrt.readthedocs.io/en/latest/usage.html
