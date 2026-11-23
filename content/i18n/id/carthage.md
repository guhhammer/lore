# carthage

Suatu alat manajer modul ketergantungan/dependensi untuk pengembangan aplikasi berbasis Cocoa.

## Usage
- `carthage update` : unduh dan bangun seluruh dependensi versi terkini, yang didaftarkan dalam berkas Cartfile
- `carthage update --platform ios` : perbarui daftar dependensi, namun hanya lakukan pembangunan untuk program iOS
- `carthage update --no-build` : perbarui daftar dependensi tanpa melakukan proses pembangunan apapun
- `carthage bootstrap` : unduh dan bangun kembali versi dependensi saat ini (tanpa memperbaruinya)
- `carthage build dependensi` : lakukan proses pembangunan ulang atas suatu dependensi

## Links
- More information: https://github.com/Carthage/Carthage
