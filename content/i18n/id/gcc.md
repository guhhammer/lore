# gcc

Praproses dan susun kode sumber C dan C++, lalu rakit dan gabungkan bersama-sama. Bagian dari GCC (GNU Compiler Collection).

## Usage
- `gcc jalan/menuju/sumber1.c jalan/menuju/sumber2.c ... -o jalan/menuju/program` : ubah beberapa sumber kode menjadi program
- `gcc jalan/menuju/sumber.c -Wall -o jalan/menuju/program` : tampilkan pesan peringatan dan galat dalam output
- `gcc jalan/menuju/sumber.c -Wall -g -Og -o jalan/menuju/program` : izinkan peringatan dan simbol debug dalam output
- `gcc jalan/menuju/sumber.c -o jalan/menuju/program -Ijalan/menuju/header -Ljalan/menuju/pustaka -lnama_pustaka` : sertakan pustaka (library) dari direktori yang berbeda
- `gcc -S jalan/menuju/sumber.c` : susun kode sumber ke dalam bahasa tingkat rendah (assembly)
- `gcc -c jalan/menuju/sumber.c` : susun kode sumber tanpa digabungkan
- `gcc jalan/menuju/sumber.c -O1|2|3|fast -o jalan/menuju/program` : optimalkan progam yang disusun agar dapat dijalankan lebih cepat
- `gcc --version` : tampilkan versi

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
