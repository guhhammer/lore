# xattr

Utilitas untuk bekerja dengan atribut sistem berkas yang diperluas.

## Usage
- `xattr -l berkas` : tampilkan daftar atribut kunci:nilai yang diperluas untuk berkas tertentu
- `xattr -w kunci_atribut nilai_atribut berkas` : tulis atribut untuk suatu berkas
- `xattr -d com.apple.quarantine berkas` : hapus atribut dari suatu berkas
- `xattr -c berkas` : hapus semua atribut yang diperluas dari suatu berkas
- `xattr -rd kunci_atribut jalan/menuju/direktori` : hapus atribut secara rekursif dalam direktori tertentu

## Links
- More information: https://keith.github.io/xcode-man-pages/xattr.1.html
