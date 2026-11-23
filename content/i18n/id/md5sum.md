# md5sum

Hitung hasil persandian hash (checksum) untuk kumpulan berkas menggunakan algoritma MD5.

## Usage
- `md5sum jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : hitung hash atas satu atau beberapa berkas menggunakan algoritma MD5
- `md5sum jalan/menuju/berkas1 jalan/menuju/berkas2 ... >jalan/menuju/berkas_manifes.md5` : hitung dan simpan daftar hasil hash beserta alamat berkas asal ke dalam suatu berkas manifes
- `perintah | md5sum` : hitung hash atas isi dari `stdin`
- `md5sum -c jalan/menuju/berkas_manifes.md5` : baca suatu berkas manifest serta lakukan verifikasi atas nilai hash dari isi berkas-berkas tersebut
- `md5sum -c --quiet jalan/menuju/berkas_manifes.md5` : hanya luarkan pesan-pesan galat ketika terjadi kegagalan atas proses verifikasi
- `md5sum --ignore-missing -c --quiet jalan/menuju/berkas_manifes.md5` : hanya luarkan pesan-pesan galat dan abaikan berkas-berkas yang tak ditemukan
- `echo nilai_hash_md5_yang_diketahui jalan/menuju/berkas | md5sum -c` : periksa keabsahan isi suatu berkas dengan nilai hash MD5 yang diketahui
