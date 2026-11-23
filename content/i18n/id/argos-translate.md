# argos-translate

Sebuah pustaka Python bersumber terbuka dengan alat baris perintah untuk melakukan alih bahasa secara luring.

## Usage
- `argospm install translate-es_en` : pasang berkas penerjemah untuk alih bahasa dari bahasa Spanyol menuju Inggris
- `argos-translate --from-lang es --to-lang en un texto corto` : terjemahkan sebagian teks dari bahasa Spanyol (`es`) menuju Inggris (`en`) (Catatan: Program ini hanya mendukung kode bahasa dengan dua huruf)
- `cat jalan/menuju/berkas.txt | argos-translate --from-lang en --to-lang hi` : terjemahkan suatu berkas teks dari bahasa Inggris menuju Hindi
- `argospm list` : tampilkan daftar seluruh pasangan penerjemah bahasa yang tersedia
- `argospm search --from-lang en` : tampilkan pasangan penerjemah yang tersedia untuk menerjemahkan teks bahasa Inggris menuju bahasa lain
- `argospm update` : perbarui berkas penerjemah bahasa yang terpasang
- `argos-translate --from-lang ar --to-lang ru صورة تساوي أكثر من ألف كلمة` : terjemahkan teks dari `ar` menuju `ru` (Catatan: Proses ini membutuhkan berkas penerjemah `translate-ar_en` dan `translate-en_ru` untuk terpasang terlebih dahulu)

## Links
- More information: https://argos-translate.readthedocs.io/en/latest/source/cli.html
