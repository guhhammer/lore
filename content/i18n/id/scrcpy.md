# scrcpy

Tampilkan layar and kontrol perangkat Android anda di dalam desktop.

## Usage
- `scrcpy` : tampilkan layar sebuah perangkat yang terhubung
- `scrcpy -S -w` : matikan layar namun cegat perangkat tujuan dari memasukan mode tidur saat sedang ditampilkan
- `scrcpy -s 0123456789abcdef|192.168.0.1:5555` : tampilkan layar perangkat tertentu berdasarkan ID atau alamat IP-nya (temukan menggunakan perintah `adb devices`)
- `scrcpy -f` : tampilkan layar dalam mode layar penuh / fullscreen
- `scrcpy -t` : tunjukkan indikator sentuhan pada perangkat fisik
- `scrcpy -r jalan/menuju/berkas.mp4` : rekam tampilan layar perangkat ke dalam file video tertentu
- `scrcpy --push-target jalan/menuju/direktori` : tentukan direktori yang akan digunakan untuk memindahkan file (non-APK) ke dalam perangkat melalui drag-and-drop

## Links
- More information: https://github.com/Genymobile/scrcpy
