# kaggle

CLI resmi untuk Kaggle yang diimplementasikan dalam Python 3.

## Usage
- `kaggle config view` : tampilkan nilai konfigurasi saat ini
- `kaggle c download kompetisi -f jalan/menuju/berkas` : unduh file tertentu dari dataset kompetisi
- `kaggle c list -s kata_kunci` : tampilkan daftar kompetisi yang sesuai dengan kata kunci pencarian
- `kaggle c files kompetisi` : tampilkan daftar file yang tersedia untuk kompetisi tertentu
- `kaggle c submit kompetisi -f jalan/ke/hasil_prediksi.csv -m "pesan"` : kirim file ke kompetisi disertai pesan
- `kaggle d list -s kata_kunci` : tampilkan daftar dataset yang sesuai dengan kata kunci pencarian
- `kaggle d download pemilik/nama_dataset` : unduh semua file dari sebuah dataset
- `kaggle k list -s kata_kunci` : tampilkan daftar kernel (notebook) yang sesuai dengan kata kunci pencarian

## Links
- More information: https://github.com/Kaggle/kaggle-api/blob/main/docs/README.md
