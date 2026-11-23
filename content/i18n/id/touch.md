# touch

Buat berkas-berkas kosong baru dan setel waktu akses dan modifikasi terhadap para berkas.

## Usage
- `touch jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : buat kumpulan berkas baru
- `touch -c -a|m jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : atur informasi waktu akses atau modifikasi pada kumpulan berkas yang telah tersedia dalam penyimpanan, dan jangan membuat (create) berkas baru jika tak tersedia dalam penyimpanan
- `touch -c -t YYYYMMDDHHMM.SS jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : atur informasi waktu terhadap kumpulan berkas, dan jangan membuat (create) berkas baru jika tak tersedia dalam penyimpanan
- `touch -c -r jalan/menuju/berkas_referensi jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : gunakan informasi waktu atas suatu berkas referensi terhadap kumpulan berkas yang diolah, dan jangan membuat (create) berkas baru jika tak tersedia dalam penyimpanan
