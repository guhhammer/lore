# dos2unix

Ubah format pengakhiran baris teks pada suatu berkas teks (plaintext) dari format DOS menuju Unix. Program ini menggantikan simbol CRLF menjadi LF.

## Usage
- `dos2unix jalan/menuju/berkas` : ganti format pengakhiran baris teks dan simpan perubahan pada berkas yang sama
- `dos2unix -n jalan/menuju/berkas jalan/menuju/berkas_baru` : ganti format namun simpan perubahan sebagai berkas baru
- `dos2unix -i jalan/menuju/berkas` : tampilkan informasi suatu berkas teks
- `dos2unix --keep-bom|add-bom|remove-bom jalan/menuju/berkas` : tetap jaga/tambahkan/hapus simbol Byte Order Mark (BOM) saat mengubah isi berkas

## Links
- More information: https://manned.org/dos2unix
