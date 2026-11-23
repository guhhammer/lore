# top

Tampilkan informasi waktu nyata dinamis tentang proses yang berjalan.

## Usage
- `top` : jalankan `top`
- `top -i` : jangan tampilkan proses-proses yang sedang diam atau tidak bernyawa (zombie)
- `top -u nama_pengguna` : hanya tampilkan kumpulan proses yang dipunyai oleh suatu pengguna
- `top -o nama_kolom` : urutkan daftar proses berdasarkan urutan data dalam kolom tertentu
- `top -Hp id_proses` : tampilkan daftar thread individu untuk suatu proses
- `top -p $(pgrep -d ',' nama_proses)` : hanya tampilkan proses yang memiliki nomor induk (PID) tertentu, masing-masing dipisah dengan tanda koma (Umumnya Anda takkan mengetahui nomor-nomornya. Perintah contoh ini secara otomatis mengambil daftar PID berdasarkan nama proses diketahui)
- `<?>` : tampilkan bantuan mengenai perintah-perintah interaktif

## Links
- More information: https://manned.org/top
