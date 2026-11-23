# pm2

Manajer proses untuk Node.js. Digunakan untuk manajemen log, pemantauan, dan pengaturan proses.

## Usage
- `pm2 start app.js --name nama_aplikasi` : memulai prooses dengan nama yang dapat digunakan untuk operasi selanjutnya
- `pm2 list` : tampilkan daftar proses
- `pm2 monit` : pantau semua proses
- `pm2 stop nama_aplikasi` : menghentikan sebuah proses
- `pm2 restart nama_aplikasi` : memulai ulang sebuah proses
- `pm2 save` : membuang semua proses dan menghidupkan mereka kembali nanti
- `pm2 resurrect` : menghidupkan kembali proses yang sebelumnya dibuang

## Links
- More information: https://pm2.keymetrics.io/docs/usage/quick-start/
