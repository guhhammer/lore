# pnpm

Manajer paket JavaScript dan Node.js yang cepat dan efisien. Mengelola proyek Node.js dan dependensi modulnya.

## Usage
- `pnpm init` : buat file `package.json` file
- `pnpm i` : unduh semua paket yang terdaftar sebagai ketergantungan/dependensi di `package.json`
- `pnpm add nama_modul@versi` : unduh versi tertentu dari sebuah paket dan menambahkan ke daftar ketergantungan di `package.json`
- `pnpm add nama_modul -D` : unduh paket dan menambahkan ke daftar ketergantungan Dev di `package.json`
- `pnpm add module_name -g` : unduh dan pasang paket secara global
- `pnpm rm module_name` : copot pemasangan dan hapus paket dari daftar ketergantungan dalam `package.json`
- `pnpm ls` : tampilkan daftar ketergantungan yang diinstal secara lokal
- `pnpm ls -g --depth 0` : tampilkan daftar modul tingkat atas yang diinstal secara global

## Links
- More information: https://pnpm.io/pnpm-cli
