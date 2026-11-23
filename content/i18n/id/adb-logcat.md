# adb logcat

Dapatkan dan simpan log sistem pada perangkat Android.

## Usage
- `adb logcat` : tampilkan log sistem pada perangkat yang terhubung saat ini
- `adb logcat -e ekspresi_reguler` : saring dan tampilkan log berdasarkan kriteria ekspresi reguler
- `adb logcat tag:mode *:S` : saring dan tampilkan log menurut tingkat mode (Verbose, Debug, Info, Warning, Error, Fatal, Silent) serta tag pada pesan-pesan log
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : tampilkan pesan-pesan log dari aplikasi berbasis React Native dalam mode Verbose, dan jangan tampilkan (Silence) pesan lainnya
- `adb logcat *:W` : tampilkan pesan-pesan log yang dikategorikan dalam tingkat mode Warning ke atas, tanpa menghiraukan jenis tag yang disaring
- `adb logcat --pid pid` : tampilkan pesan-pesan log dari proses tertentu (menurut kode PID proses tersebut)
- `adb logcat --pid $(adb shell pidof -s paket)` : tampilkan pesan-pesan log dari aplikasi tertentu (menurut package identifier seperti `com.example.myapp`)
- `adb logcat -v color` : tampilkan log sistem secara warna-warni (biasanya digunakan untuk menyaring pesan-pesan log)

## Links
- More information: https://developer.android.com/tools/logcat
