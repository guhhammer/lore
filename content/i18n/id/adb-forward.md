# adb forward

Teruskan akses soket menuju suatu perangkat atau emulator Android.

## Usage
- `adb forward tcp:port_lokal tcp:port_jarak_jauh` : teruskan akses suatu port TCP menuju emulator atau perangkat yang terhubung saat ini
- `adb -s id_perangkat forward tcp:port_lokal tcp:port_remote` : teruskan akses suatu port TCP menuju suatu emulator atau perangkat secara spesifik (berdasarkan nomor induk / serial perangkat)
- `adb forward --list` : tampilkan daftar seluruh pengaturan terusan
- `adb forward --remove tcp:port_lokal` : hapus suatu aturan terusan
- `adb forward --remove-all` : hapus seluruh pengaturan terusan

## Links
- More information: https://developer.android.com/tools/adb
