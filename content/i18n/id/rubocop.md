# rubocop

Analisa berkas Ruby.

## Usage
- `rubocop` : periksa semua berkas dalam direktori saat ini (termasuk direktori-direktori di dalamnya)
- `rubocop jalan/menuju/berkas_atau_direktori1 jalan/menuju/berkas_atau_direktori2 ...` : periksa satu atau lebih berkas atau direktori secara khusus
- `rubocop --out jalan/menuju/berkas` : tulis output ke berkas
- `rubocop --show-cops` : lihat daftar cop (aturan-aturan dalam menganalisa)
- `rubocop --except cop1 cop2 ...` : kecualikan kumpulan cop dalam proses analisa
- `rubocop --only cop1 cop2 ...` : jalankan hanya beberapa cop
- `rubocop --auto-correct` : perbaiki berkas secara otomatis (fitur percobaan)

## Links
- More information: https://docs.rubocop.org/rubocop/latest/usage/cli_reference.html
