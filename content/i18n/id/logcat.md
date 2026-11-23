# logcat

Tampilkan dan simpan log sistem.

## Usage
- `logcat` : tampilkan log sistem
- `logcat -f jalan/menuju/berkas` : simpan log sistem di dalam sebuah file
- `logcat --regex regular_expression` : saring informasi log berdasarkan sintaks ekspresi reguler (`regex`) tertentu
- `logcat --pid pid` : tampilkan log untuk nomor induk (PID) program yang sedang dijalankan
- `logcat --pid $(pidof -s paket)` : tampilkan log untuk (kemasan) aplikasi yang sedang dijalankan

## Links
- More information: https://developer.android.com/tools/logcat
