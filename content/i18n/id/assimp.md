# assimp

Klien baris perintah untuk pustaka Open Asset Import Library. Mendukung pemuatan 40+ format file 3D, dan mengekspor ke beberapa format 3D populer.

## Usage
- `assimp listext` : tampilkan daftar format berkas impor yang didukung
- `assimp listexport` : tampilkan daftar format berkas ekspor yang didukung
- `assimp export berkas_masukan.stl berkas_luaran.obj` : ubah isi suatu berkas menuju salah satu dari format berkas ekspor/luaran yang didukung, menggunakan daftar parameter bawaan
- `assimp export berkas_masukan.stl berkas_luaran.obj kumpulan_parameter` : ubah isi suatu berkas menggunakan kumpulan parameter kustom (daftar parameter yang tersedia dapat dilihat pada berkas `dox_cmd.h` pada kode sumber assimp)
- `assimp info jalan/menuju/berkas` : tampilkan ringkasan isi suatu berkas objek 3D
- `assimp help` : tampilkan bantuan
- `assimp subperintah --help` : tampilkan bantuan atas suatu subperintah

## Links
- More information: https://manned.org/assimp
