# apkleaks

Pindai berkas APK (aplikasi Android) untuk mencari URI, alur pemanggilan (endpoint), dan konfigurasi rahasia. Catatan: APKLeaks menggunakan `jadx` untuk membongkar kode aplikasi Android.

## Usage
- `apkleaks -f jalan/menuju/berkas.apk` : pindai berkas (file) APK untuk mencari daftar endpoint dan kode konfigurasi rahasia
- `apkleaks -f jalan/menuju/berkas.apk -o jalan/menuju/berkas_output.txt` : pindai dan simpan luaran (output) ke dalam suatu berkas
- `apkleaks -f jalan/menuju/berkas.apk -a "--threads-count 5 --deobf"` : berikan argumen perintah tambahan untuk `jadx`

## Links
- More information: https://github.com/dwisiswant0/apkleaks
