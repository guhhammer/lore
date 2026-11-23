# clang

Susun kode sumber C, C++, dan Objective-C. Dapat dipakai sebagai pengganti mutlak (drop-in) bagi GCC. Bagian dari LLVM.

## Usage
- `clang jalan/menuju/sumber1.c jalan/menuju/sumber2.c ... -o jalan/menuju/program` : ubah suatu berkas kode sumber menjadi program
- `clang jalan/menuju/sumber.c -Wall -o jalan/menuju/program` : tampilkan pesan peringatan dan galat dalam output
- `clang jalan/menuju/sumber.c -Wall -g -Og -o jalan/menuju/program` : izinkan peringatan dan simbol debug dalam output
- `clang jalan/menuju/sumber.c -o jalan/menuju/program -Ijalan/menuju/header -Ljalan/menuju/pustaka -lnama_pustaka` : sertakan pustaka (library) dari direktori yang berbeda
- `clang -S -emit-llvm jalan/menuju/sumber.c -o jalan/menuju/program.ll` : susun kode sumber menjadi program dalam format LLVM Intermediate Representation (IR)
- `clang -c jalan/menuju/sumber.c` : susun kode sumber tanpa digabungkan
- `clang jalan/menuju/sumber.c -O1|2|3|fast -o jalan/menuju/program` : optimalkan progam yang disusun agar dapat dijalankan lebih cepat
- `clang --version` : tampilkan versi

## Links
- More information: https://clang.llvm.org/docs/ClangCommandLineReference.html
