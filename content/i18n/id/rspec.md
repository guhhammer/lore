# rspec

Kerangka pengujian kode Ruby berbasis Ruby dan pola pengembangan berbasis kebiasaan (behavior-driven development).

## Usage
- `rspec --init` : buat suatu berkas konfigurasi `.rspec` dan berkas pendukung spesifikasi pengujian (spec helper)
- `rspec` : jalankan semua pengujian menurut berkas-berkas spesifikasi
- `rspec jalan/menuju/direktori` : jalankan pengujian menurut berkas-berkas spesifikasi dalam direktori khusus
- `rspec jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : jalankan beberapa pengujian menurut kumpulan berkas spesifikasi
- `rspec jalan/menuju/berkas:83` : jalankan kasus khusus dalam pengujian menurut berkas-berkas spesifikasi (misalnya tes yang ada di baris 83)
- `rspec --seed angka_seed` : jalankan tes dengan seed khusus (untuk pengujian berbasis randomisasi)

## Links
- More information: https://rspec.info/features/3-13/rspec-core/command-line/
