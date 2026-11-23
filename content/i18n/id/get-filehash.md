# get-filehash

Hitung hasil persandian hash (checksum) untuk suatu berkas. Catatan: Perintah ini hanya dapat digunakan dalam PowerShell.

## Usage
- `Get-FileHash jalan\menuju\berkas` : hitung hash atas suatu berkas menggunakan algoritma SHA256
- `Get-FileHash jalan\menuju\berkas -Algorithm SHA1|SHA384|SHA256|SHA512|MD5` : hitung hash atas suatu berkas menggunakan algoritma tertentu
- `Get-FileHash jalan\menuju\berkas -Algorithm SHA1|SHA384|SHA256|SHA512|MD5` : hitung hash atas suatu berkas menggunakan algoritma usang tertentu (tidak tersedia dalam PowerShell versi 6 ke atas)
- `(Get-FileHash jalan\menuju\berkas -Algorithm SHA1|SHA256|SHA384|SHA512|MD5).Hash -eq "nilai_hash_blake2_yang_diketahui"` : periksa keabsahan isi suatu berkas dengan nilai hash yang diketahui
