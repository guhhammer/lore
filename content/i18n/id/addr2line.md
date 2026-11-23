# addr2line

Konversi alamat dari file biner menjadi nama file dan nomor baris.

## Usage
- `addr2line -e path/ke/executable address` : tampilkan nama file dan nomor baris dari source code berdasarkan alamat instruksi dalam sebuah file executable
- `addr2line -e path/ke/executable -f address` : tampilkan nama fungsi, nama file, dan nomor baris
- `addr2line -e path/ke/executable -f -C address` : lakukan demangle pada nama fungsi untuk kode C++

## Links
- More information: https://manned.org/addr2line
