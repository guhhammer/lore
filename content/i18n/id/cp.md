# cp

Salin berkas dan direktori.

## Usage
- `cp jalan/menuju/berkas_sumber.ext jalan/menuju/berkas_tujuan.ext` : salin berkas ke lokasi lain
- `cp jalan/menuju/berkas_sumber.ext jalan/menuju/direktori_tujuan` : salin berkas ke direktori lain, dengan nama yang sama
- `cp -r jalan/menuju/direktori_sumber jalan/menuju/direktori_tujuan` : salin sebuah direktori secara rekursif beserta isinya ke lokasi lain (jika tujuan sudah ada, direktori tersebut disalin ke dalamnya)
- `cp -vr jalan/menuju/direktori_sumber jalan/menuju/direktori_tujuan` : salin sebuah direktori secara rekursif beserta isinya, dengan menampilkan berkas-berkas ketika disalin (mode verbose)
- `cp -t path/to/direktori_tujuan jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : salin lebih dari satu berkas menuju suatu direktori
- `cp -i *.txt jalan/menuju/direktori_tujuan` : salin berkas-berkas teks ke lokasi lain, dalam mode interaktif (menampilkan pertanyaan sebelum menimpa)
- `cp -L tautan jalan/menuju/direktori_tujuan` : salin tautan simbolis sebelum menyalin
- `cp -t path/to/direktori_tujuan jalan/menuju/berkas_atau_direktori1 jalan/menuju/berkas_atau_direktori2 ...` : gunakan argumen pertama sebagai direktori tujuan (berguna untuk perintah seperti `xargs ... | cp -t destination_directory`)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
