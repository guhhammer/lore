# adb install

Android Debug Bridge Install: Memasang paket ke emulator Android atau perangkat Android terhubung.

## Usage
- `adb install jalan/menuju/berkas.apk` : pasang aplikasi Android ke emulator/perangkat
- `adb -s nomor_serial install jalan/menuju/berkas.apk` : pasang aplikasi Android menuju emulator/perangkat tertentu (berdasarkan nomor serial yang didapatkan dari `adb devices`, mengesampingkan nilai `$ANDROID_SERIAL`)
- `adb install -r jalan/menuju/berkas.apk` : pasang ulang aplikasi yang sudah ada, menjaga datanya
- `adb install -d jalan/menuju/berkas.apk` : pasang aplikasi Android versi lawas dari aplikasi Android yang sudah terpasang pada perangkat (khusus aplikasi debug)
- `adb install -g jalan/menuju/berkas.apk` : berikan semua izin yang terdaftar di manifest aplikasi
- `adb install --fastdeploy jalan/menuju/berkas.apk` : perbarui langsung paket terinstal dengan hanya memperbarui bagian dari APK yang berubah

## Links
- More information: https://developer.android.com/tools/adb
