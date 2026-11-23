# apk

Alat manajemen paket Alpine Linux.

## Usage
- `apk upgrade -U` : perbarui indeks repositori dan upgrade semua paket
- `apk update` : perbarui indeks repositori saja
- `apk add paket` : pasang suatu paket baru
- `apk del paket` : hapus pemasangan paket
- `apk fix paket` : perbaiki/pasang ulang paket tanpa memodifikasi dependensi utama
- `apk search -v keyword` : cari paket dengan kata kunci pada namanya dan tampilkan hasil beserta deskripsi
- `apk search -d keyword` : cari paket dengan kata kunci pada deskripsinya
- `apk info paket` : tampilkan informasi tentang paket tertentu

## Links
- More information: https://wiki.alpinelinux.org/wiki/Alpine_Package_Keeper
