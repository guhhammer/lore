# nvm

Memasang, lepas, atau ganti versi Node.js yang dipakai. Mendukung nomor versi seperti "12.8" or "v16.13.1", dan label versi seperti "stable", "system", dsb.

## Usage
- `nvm install versi_node_js` : pasang suatu versi Node.js
- `nvm use versi_node_js` : gunakan suatu versi Node.js untuk sesi saat ini
- `nvm alias default versi_node_js` : tentukan versi default Node.js untuk sesi-sesi berikutnya
- `nvm list` : tunjukkan daftar versi Node.js yang tersedia dan yang disetel sebagai default
- `nvm uninstall versi_node_js` : hapus pemasangan versi Node.js yang terpasang melalui `nvm`
- `nvm run versi_node_js --version` : jalankan interpreter (REPL) Node.js dengan versi tertentu
- `nvm exec versi_node_js node app.js` : jalankan suatu berkas atau program JavaScript di dalam Node.js versi tertentu

## Links
- More information: https://github.com/nvm-sh/nvm#usage
