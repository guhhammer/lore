# find

Mencari file atau direktori di dalam sebuah struktur direktori (directory tree) secara rekursif.

## Usage
- `find jalan/menuju/direktori -name '*.ext'` : cari berkas berdasarkan ekstensi
- `find jalan/menuju/direktori -path '*/lokasi/*/*.ext' -or -name '*pola*'` : cari berkas yang cocok dengan beberapa pola nama/lokasi
- `find jalan/menuju/direktori -type d -iname '*lib*'` : cari direktori yang cocok dengan nama tertentu, dalam mode tidak peka huruf (case-insensitive)
- `find jalan/menuju/direktori -name '*.py' -not -path '*/site-packages/*'` : cari berkas yang cocok dengan pola tertentu, dengan mengecualikan lokasi spesifik
- `find jalan/menuju/direktori -maxdepth 1 -size +500k -size -10M` : cari berkas dengan rentang ukuran tertentu dengan membatasi kedalaman rekursif hingga "1"
- `find jalan/menuju/direktori -name '*.ext' -exec wc -l {} \;` : jalankan perintah untuk setiap berkas/fail (gunakan `{}` di dalam perintah untuk mengakses nama berkas/fail)
- `find jalan/menuju/direktori -daystart -mtime -1 -exec tar -cvf arsip.tar {} \+` : cari semua berkas/fail yang diubah hari ini dan teruskan hasilnya sebagai argumen ke satu perintah
- `find jalan/menuju/direktori -type f|d -empty -delete -print` : cari berkas/fail atau direktori kosong lalu hapus dengan menampilkan prosesnya (verbose)

## Links
- More information: https://manned.org/find
