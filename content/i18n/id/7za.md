# 7za

Pengarsip berkas dengan rasio kompresi yang tinggi. Serupa dengan `7z` namun mendukung lebih sedikit format berkas arsip dan dapat digunakan lintas sistem operasi.

## Usage
- `7za a jalan/menuju/arsip.7z jalan/menuju/berkas_atau_direktori` : tambahkan sebuah berkas atau direktori ke dalam arsip baru atau saat ini
- `7za a jalan/menuju/arsip_terenkripsi.7z -pkata sandi -mhe=on jalan/menuju/arsip.7z` : sandikan berkas arsip saat ini (beserta nama-nama berkas yang terkandung di dalamnya)
- `7za x jalan/menuju/arsip.7z` : extrak berkas arsip dengan mempertahankan struktur direktori asli
- `7za x jalan/menuju/arsip.7z -ojalan/menuju/direktori` : extrak berkas arsip ke dalam direktori yang ditentukan
- `7za x jalan/menuju/arsip.7z -so` : extrak berkas arsip menuju `stdout`
- `7za a -t7z|bzip2|gzip|lzip|tar|... jalan/menuju/arsip.7z jalan/menuju/berkas_atau_direktori` : arsipkan berkas atau direktori menggunakan format arsip tertentu
- `7za l jalan/menuju/arsip.7z` : lihat daftar isi dari sebuah berkas arsip
- `7za a jalan/menuju/arsip.7z -mx=0|1|3|5|7|9 jalan/menuju/berkas_atau_direktori` : atur tingkat kompresi pada berkas arsip (tingkat lebih tinggi akan mengurangi lebih banyak beban berkas, namun lebih lambat)

## Links
- More information: https://manned.org/7za
