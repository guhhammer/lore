# astyle

Program indenter, formatter, dan beautifier untuk berkas kode bahasa pemrograman C, C++, C#, dan Java. Setelah dijalankan, berkas asli akan disalin menuju nama berkas yang ditambahkan dengan akhiran `.orig`.

## Usage
- `astyle berkas_sumber` : terapkan pengayaan penulisan 4 karakter spasi per indent tanpa mengubah format kode
- `astyle -A2 jalan/menuju/berkas` : terapkan pengayaan penulisan kode Java dengan posisi kurung kurawal terpasang
- `astyle -A1 jalan/menuju/berkas` : terapkan pengayaan penulisan kode allman dengan posisi kurung kurawal dalam baris terpisah
- `astyle -s2..20 jalan/menuju/berkas` : terapkan indentasi kustom menggunakan karakter spasi. Pilih jumlah antara 2 dan 20 karakter
- `astyle -t2..20 jalan/menuju/berkas` : terapkan indentasi kustom menggunakan karakter tab. Pilih jumlah antara 2 dan 20 karakter

## Links
- More information: https://manned.org/astyle
