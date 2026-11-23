# clang++

Susun kode C++. Bagian dari LLVM.

## Usage
- `clang++ jalan/menuju/sumber1.cpp jalan/menuju/sumber2.cpp ... -o jalan/menuju/program` : ubah suatu berkas kode sumber menjadi program
- `clang++ jalan/menuju/sumber.cpp -Wall -o jalan/menuju/program` : tampilkan pesan peringatan dan galat dalam output
- `clang++ jalan/menuju/sumber.cpp -Wall -g -Og -o jalan/menuju/program` : izinkan peringatan dan simbol debug dalam output
- `clang++ jalan/menuju/sumber.cpp -std=c++20 -o jalan/menuju/program` : pilih standar bahasa untuk dikompilasi
- `clang++ jalan/menuju/sumber.cpp -o jalan/menuju/program -Ijalan/menuju/header -Ljalan/menuju/pustaka -lnama_pustaka` : sertakan pustaka (library) dari direktori yang berbeda
- `clang++ -S -emit-llvm jalan/menuju/sumber.cpp -o jalan/menuju/program.ll` : susun kode sumber menjadi program dalam format LLVM Intermediate Representation (IR)
- `clang++ jalan/menuju/sumber.cpp -O1|2|3|fast -o jalan/menuju/program` : optimalkan progam yang disusun agar dapat dijalankan lebih cepat
- `clang++ --version` : tampilkan versi

## Links
- More information: https://clang.llvm.org/docs/UsersManual.html#command-line-options
