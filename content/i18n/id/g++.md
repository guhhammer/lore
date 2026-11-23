# g++

Susun kode sumber C++. Bagian dari GCC (GNU Compiler Collection).

## Usage
- `g++ jalan/menuju/sumber1.c jalan/menuju/sumber2.cpp ... -o jalan/menuju/program` : ubah suatu berkas kode sumber menjadi program
- `g++ jalan/menuju/sumber.cpp -Wall -o jalan/menuju/program` : tampilkan pesan peringatan dan galat dalam output
- `g++ jalan/menuju/sumber.cpp -Wall -g -Og -o jalan/menuju/program` : izinkan peringatan dan simbol debug dalam output
- `g++ jalan/menuju/sumber.cpp -std=c++98|c++11|c++14|c++17 -o jalan/menuju/program` : pilih standar bahasa untuk dikompilasi (C++98/C++11/C++14/C++17)
- `g++ jalan/menuju/sumber.cpp -o jalan/menuju/program -Ijalan/menuju/header -Ljalan/menuju/pustaka -lnama_pustaka` : sertakan pustaka (library) dari direktori yang berbeda
- `g++ -c jalan/menuju/sumber1.cpp jalan/menuju/sumber2.cpp ... && g++ -o jalan/menuju/program jalan/menuju/sumber1.o jalan/menuju/sumber2.o ...` : susun dan gabungkan beberapa berkas kode sumber menjadi suatu berkas program biner
- `g++ jalan/menuju/sumber.cpp -O1|2|3|fast -o jalan/menuju/program` : optimalkan progam yang disusun agar dapat dijalankan lebih cepat
- `g++ --version` : tampilkan versi

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/C_002b_002b-Dialect-Options.html
