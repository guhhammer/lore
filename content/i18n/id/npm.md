# npm

Manajer paket JavaScript dan Node.js. Mengelola proyek Node.js dan dependensi modulnya.

## Usage
- `npm init -y` : buat berkas `package.json` dengan nilai pengaturan default (hapus `--yes` untuk membuatnya secara interaktif)
- `npm i` : unduh semua paket yang terdaftar sebagai ketergantungan/dependency dalam `package.json`
- `npm i nama_modul@versi` : unduh versi tertentu dari sebuah paket dan menambahkan ke daftar ketergantungan di `package.json`
- `npm i nama_modul -D` : unduh dan tambahkan paket ke daftar ketergantungan pengembangan (dev dependency) di `package.json`
- `npm i nama_modul -g` : unduh dan pasang paket ke secara global
- `npm r nama_modul` : copot pemasangan dan hapus paket dari daftar ketergantungan dalam `package.json`
- `npm ls` : tampilkan daftar paket ketergantungan yang dipasang secara lokal
- `npm ls -g --depth 0` : tampilkan daftar paket tingkat atas yang diinstal secara global

## Links
- More information: https://docs.npmjs.com/cli/npm/
