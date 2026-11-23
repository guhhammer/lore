# bun build

Lakukan proses pengemasan berkas program JavaScript dan TypeScript dengan pengemas Bun yang cepat.

## Usage
- `bun build jalan/menuju/titik_masuk.ts --outfile jalan/menuju/luaran.js` : kemas suatu berkas titik masuk program menuju berkas luaran/output
- `bun build jalan/menuju/berkas1.ts jalan/menuju/berkas2.ts ... --outdir jalan/menuju/dist` : kemas beberapa berkas menuju suatu direktori luaran
- `bun build jalan/menuju/titik_masuk.ts --outfile jalan/menuju/luaran.js --sourcemap` : kemas pula peta kode sumber untuk memudahkan proses awakutu/debugging
- `bun build jalan/menuju/titik_masuk.ts --outfile jalan/menuju/luaran.js --minify` : kemas dan perkecil berkas luaran untuk produksi
- `bun build jalan/menuju/titik_masuk.ts --outfile jalan/menuju/luaran.js --target browser|bun|node` : kemas dengan menujukan lingkungan eksekusi/runtime program
- `bun build jalan/menuju/titik_masuk.ts --compile --outfile jalan/menuju/executable` : kemas ke dalam suatu berkas program executable
- `bun build jalan/menuju/titik_masuk.ts --outfile jalan/menuju/luaran.js --watch` : terus awasi kode sumber dan kemas ulang bila terdapat perubahan
- `bun build jalan/menuju/titik_masuk.ts --outfile jalan/menuju/luaran.js -e react react-dom` : masukkan juga kode modul-modul eksternal yang dipakai dalam program

## Links
- More information: https://bun.com/docs/bundler
