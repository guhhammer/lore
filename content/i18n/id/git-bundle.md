# git bundle

Bungkus seluruh objek dan referensi internal Git ke dalam suatu berkas arsip.

## Usage
- `git bundle create jalan/menuju/berkas.bundle nama_cabang` : buat sebuah berkas (bundle) dengan seluruh objek dan referensi Git pada cabang tertentu
- `git bundle create jalan/menuju/berkas.bundle --all` : bungkus objek dan referensi untuk seluruh cabang
- `git bundle create jalan/menuju/berkas.bundle -5 HEAD` : bungkus objek dan referensi untuk lima komit terakhir pada cabang saat ini
- `git bundle create jalan/menuju/berkas.bundle --since 7.days HEAD` : bungkus objek dan referensi untuk perubahan sejak 7 hari terakhir
- `git bundle verify jalan/menuju/berkas.bundle` : cek apakah suatu berkas bundle bersifat valid dan dapat diaplikasikan ke dalam repositori saat ini
- `git bundle unbundle jalan/menuju/berkas.bundle` : cetak daftar berkas referensi yang terkandung dalam berkas bundle menuju `stdout`
- `git pull jalan/menuju/berkas.bundle nama_cabang` : buka dan pakai isi bungkusan untuk suatu cabang pada repositori saat ini
- `git clone jalan/menuju/berkas.bundle` : buat sebuah repositori baru dari suatu berkas bundle

## Links
- More information: https://git-scm.com/docs/git-bundle
