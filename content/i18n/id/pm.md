# pm

Manajer paket aplikasi Android.

## Usage
- `pm list packages` : tampilkan daftar paket yang terpasang
- `pm install /jalan/menuju/aplikasi.apk` : pasang suatu paket aplikasi dari alamat tertentu
- `pm uninstall paket` : hapus suatu paket aplikasi dari perangkat Android
- `pm clear paket` : hapus seluruh data aplikasi untuk suatu paket
- `pm enable paket_atau_kelas` : aktifkan suatu paket atau komponen
- `pm disable-user paket_atau_kelas` : nonaktifkan suatu paket atau komponen
- `pm grant paket android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : berikan suatu ijin untuk suatu aplikasi
- `pm revoke paket android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : cabut suatu ijin untuk suatu aplikasi

## Links
- More information: https://developer.android.com/tools/adb#pm
