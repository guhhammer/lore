# dumpsys

Berikan informasi tentang layanan (daemon) sistem milik Android. Perintah ini hanya dapat dijalankan melalui `adb shell`.

## Usage
- `dumpsys` : tampilkan informasi diagnostik terhadap seluruh layanan sistem Android
- `dumpsys layanan` : tampilkan informasi diagnostik untuk layanan sistem tertentu
- `dumpsys -l` : tampilkan daftar layanan sistem yang diketahui oleh `dumpsys`
- `dumpsys layanan -h` : tampilkan daftar argumen yang diterima oleh sebuah layanan sistem
- `dumpsys --skip layanan` : kecualikan layanan sistem tertentu dari informasi diagnostik yang ditampilkan
- `dumpsys -t detik` : tetapkan periode waktu habis dalam hitungan detik (10 detik secara default)

## Links
- More information: https://developer.android.com/tools/dumpsys
