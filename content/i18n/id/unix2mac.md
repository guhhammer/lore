# unix2mac

Ubah format pengakhiran baris teks pada suatu berkas teks (plaintext) dari format Unix menuju macOS. Program ini menggantikan simbol LF menjadi CR.

## Usage
- `unix2mac jalan/menuju/berkas` : ganti format pengakhiran baris teks dan simpan perubahan pada berkas yang sama
- `unix2mac -n jalan/menuju/berkas jalan/menuju/berkas_baru` : ganti format namun simpan perubahan sebagai berkas baru
- `unix2mac -i jalan/menuju/berkas` : tampilkan informasi suatu berkas teks
- `unix2mac --keep-bom|add-bom|remove-bom jalan/menuju/berkas` : tetap jaga/tambahkan/hapus simbol Byte Order Mark (BOM) saat mengubah isi berkas

## Links
- More information: https://manned.org/unix2mac
