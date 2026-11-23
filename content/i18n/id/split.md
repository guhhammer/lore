# split

Memisahkan sebuah file menjadi beberapa bagian.

## Usage
- `split -l 10 jalan/menuju/berkas` : memisahkan sebuah file, tiap bagian memiliki 10 baris (kecuali di bagian terakhir)
- `split -n 5 jalan/menuju/berkas` : memisahkan sebuah file menjadi 5 file. Dibagi sehingga masing-masing bagian memiliki ukuran yang sama (kecuali di bagian terakhir)
- `split -b 512 jalan/menuju/berkas` : memisahkan sebuah file dengan ukuran 512 byte masing-masing bagiannya (kecuali di bagian terakhir; gunakan 512k untuk kilobyte dan 512m untuk megabytes)
- `split -C 512 jalan/menuju/berkas` : memisahkan sebuah file dengan ukuran paling banyak 512 byte masing-masing bagiannya tanpa memotong baris
