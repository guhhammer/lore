# sha256sum

Hitung hasil persandian hash (checksum) untuk kumpulan berkas menggunakan algoritma SHA256.

## Usage
- `sha256sum jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : hitung hash atas satu atau beberapa berkas menggunakan algoritma SHA256
- `sha256sum jalan/menuju/berkas1 jalan/menuju/berkas2 ... >jalan/menuju/berkas_manifes.sha256` : hitung dan simpan daftar hasil hash beserta alamat berkas asal ke dalam suatu berkas manifes
- `perintah | sha256sum` : hitung hash atas isi dari `stdin`
- `sha256sum -c jalan/menuju/berkas_manifes.sha256` : baca suatu berkas manifest serta lakukan verifikasi atas nilai hash dari isi berkas-berkas tersebut
- `sha256sum -c --quiet jalan/menuju/berkas_manifes.sha256` : hanya luarkan pesan-pesan galat ketika terjadi kegagalan atas proses verifikasi
- `sha256sum --ignore-missing -c --quiet jalan/menuju/berkas_manifes.sha256` : hanya luarkan pesan-pesan galat dan abaikan berkas-berkas yang tak ditemukan
- `echo nilai_hash_sha256_yang_diketahui jalan/menuju/berkas | sha256sum -c` : periksa keabsahan isi suatu berkas dengan nilai hash SHA256 yang diketahui

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html
