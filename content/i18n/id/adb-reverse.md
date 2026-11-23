# adb reverse

Android Debug Bridge Reverse: membalik koneksi socket dari perangkat atau emulator Android yang terhubung.

## Usage
- `adb reverse --list` : tampilkan daftar semua koneksi socket terbalik dari emulator dan perangkat
- `adb reverse tcp:port_jarak_jauh tcp:port_lokal` : balikkan akses port TCP dari emulator/perangkat menuju localhost
- `adb -s id_perangkat reverse tcp:remote_port tcp:local_port` : balikkan akses port TCP dari emulator/perangkat spesifik (berdasarkan nomor induk / serial perangkat) menuju localhost
- `adb reverse --remove tcp:port_jarak_jauh` : lepaskan koneksi socket terbalik dari emulator/perangkat
- `adb reverse --remove-all` : lepaskan semua koneksi socket terbalik dari semua emulator dan perangkat

## Links
- More information: https://developer.android.com/tools/adb
