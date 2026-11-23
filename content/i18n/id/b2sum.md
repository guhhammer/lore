# b2sum

Hitung hasil persandian hash (checksum) untuk kumpulan berkas menggunakan algoritma BLAKE2.

## Usage
- `b2sum jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : hitung hash atas satu atau beberapa berkas menggunakan algoritma BLAKE2
- `b2sum jalan/menuju/berkas1 jalan/menuju/berkas2 ... >jalan/menuju/berkas_manifes.b2` : hitung dan simpan daftar hasil hash beserta alamat berkas asal ke dalam suatu berkas manifes
- `perintah | b2sum` : hitung hash atas isi dari `stdin`
- `b2sum -c jalan/menuju/berkas_manifes.b2` : baca suatu berkas manifest serta lakukan verifikasi atas nilai hash dari isi berkas-berkas tersebut
- `b2sum -c --quiet jalan/menuju/berkas_manifes.b2` : hanya luarkan pesan-pesan galat ketika terjadi kegagalan atas proses verifikasi
- `b2sum --ignore-missing -c --quiet jalan/menuju/berkas_manifes.b2` : hanya luarkan pesan-pesan galat dan abaikan berkas-berkas yang tak ditemukan
- `echo nilai_hash_blake2_yang_diketahui jalan/menuju/berkas | b2sum -c` : periksa keabsahan isi suatu berkas dengan nilai hash BLAKE2 yang diketahui
