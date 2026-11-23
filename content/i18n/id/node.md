# node

Platform JavaScript sisi server (Node.js).

## Usage
- `node jalan/menuju/berkas` : jalankan berkas program JavaScript
- `node` : jalankan sebuah REPL (shell interaktif)
- `node --watch jalan/menuju/berkas` : jalankan berkas program dan jalankan ulang saat isi dari berkas tersebut terubah (membutuhkan Node.js versi 18.11+)
- `node -e "kode"` : evaluasi kode JavaScript dengan memberikanya sebagai sebuah argument
- `node -p "process.versions"` : evaluasi kode dan cetak hasil, berguna untuk melihat versi dependesni node
- `node --no-lazy --inspect-brk jalan/menuju/berkas` : aktifkan inspector, yang akan menjeda eksekusi sampai debugger terhubung segera setelah kode sumber sepenuhnya terparser

## Links
- More information: https://nodejs.org/docs/latest/api/cli.html#options
