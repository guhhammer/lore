# adb shell

Android Debug Bridge Shell: Menjalankan perintah shell jarak jauh pada emulator Android atau perangkat Android terhubung.

## Usage
- `adb shell` : mulaikan shell interaktif jarak jauh di emulator/perangkat
- `adb shell getprop` : dapatkan semua properti dari emulator/perangkat
- `adb shell pm reset-permissions` : kembalikan semua izin runtime ke default
- `adb shell pm revoke paket izin` : cabut izin berbahaya dari sebuah aplikasi
- `adb shell input keyevent keycode` : picukan sebuah peristiwa penting
- `adb shell pm clear paket` : kosongkan data aplikasi pada emulator/perangkat
- `adb shell am start -n paket/aktivitas` : mulaikan aktivitas pada emulator/perangkat
- `adb shell am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : mulaikan aktivitas beranda pada emulator/perangkat

## Links
- More information: https://developer.android.com/tools/adb
