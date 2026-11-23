# bc

Suatu bahasa kalkulator presisi.

## Usage
- `bc` : jalankan sesi interaktif baru
- `bc -i -l` : jalankan sesi interaktif dengan memuat pustaka dasar matematika
- `echo '5 / 3' | bc` : lakukan penghitungan terhadap suatu pernyataan
- `bc jalan/menuju/skrip.bc` : jalankan suatu berkas skrip
- `echo 'scale = 10; 5 / 3' | bc` : hitung hasil suatu pernyataan dengan skala desimal tertentu
- `echo 's|c|a|l|e(1)' | bc -l` : hitung hasil sinus, cosinus, arctangen, atau logaritma/eksponensi natural atas suatu bilangan menggunakan `mathlib`
- `echo "define factorial(n) { if (n <= 1) return 1; return n*factorial(n-1); }; factorial(10)" | bc` : jalankan suatu skrip perhitungan faktorial secara sebaris

## Links
- More information: https://manned.org/bc
