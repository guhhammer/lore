# adb

Android Debug Bridge: berkomunikasi dengan emulator Android atau perangkat Android terhubung. Beberapa subperintah seperti `shell` mempunyai dokumentasi terpisah.

## Usage
- `adb start-server` : periksa apakah proses server adb telah dimulai dan memulainya
- `adb kill-server` : hentikan proses server adb
- `adb shell` : mulai shell jarak jauh pada emulator/perangkat tujuan
- `adb install -r jalan/menuju/berkas.apk` : pasang suatu aplikasi Android menuju emulator/perangkat tujuan
- `adb pull jalan/menuju/berkas_atau_direktori_perangkat jalan/menuju/direktori_lokal_tujuan` : salin berkas/direktori dari perangkat tujuan
- `adb push jalan/menuju/berkas_atau_direktori_lokal jalan/menuju/direktori_perangkat_tujuan` : salin berkas/direktori menuju perangkat tujuan
- `adb devices` : tampilkan daftar perangkat yang terhubung
- `adb -s device_id shell` : tentukan perangkat yang diinstruksikan (berdasarkan nomor induk / Device id) jika terdapat lebih dari satu perangkat yang terhubung secara bersamaan

## Links
- More information: https://developer.android.com/tools/adb
