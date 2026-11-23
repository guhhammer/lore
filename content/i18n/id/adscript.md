# adscript

Penyusun (compiler) untuk file Adscript.

## Usage
- `adscript --output jalan/menuju/berkas.o jalan/menuju/berkas_input.adscript` : susun suatu file menjadi file objek
- `adscript --executable --output jalan/menuju/berkas jalan/menuju/berkas_input.adscript` : susun dan gabungkan file menjadi sebuah file program mandiri
- `adscript --llvm-ir --output jalan/menuju/berkas.ll jalan/menuju/berkas_input.adscript` : susun sebuah file menggunakan kode LLVM IR daripada kode mesin yang sesungguhnya
- `adscript --target-triple i386-linux-elf --output jalan/menuju/berkas.o jalan/menuju/berkas_input.adscript` : susun menjadi file objek untuk arsitektur CPU atau sistem operasi yang berbeda dengan mesin saat ini (cross-compile)

## Links
- More information: https://github.com/Amplus2/Adscript
