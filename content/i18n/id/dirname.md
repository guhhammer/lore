# dirname

Hapus bagian nama berkas dari sebuah alamat berkas, untuk menentukan alamat direktori asal berkas.

## Usage
- `dirname jalan/menuju/berkas_atau_direktori` : tentukan alamat direktori dari suatu alamat
- `dirname jalan/menuju/berkas_atau_direktori1 jalan/menuju/berkas_atau_direktori2 ...` : tentukan alamat direktori dari berbagai alamat
- `dirname -z jalan/menuju/berkas_atau_direktori1 jalan/menuju/berkas_atau_direktori2 ...` : gunakan karakter NUL daripada baris baru untuk memisah luaran antar alamat (berguna jika digunakan bersamaan dengan `xargs`)
