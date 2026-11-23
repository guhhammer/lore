# arduino

Arduino Studio - Sebuah alat pengembangan piranti lunak (IDE) bagi platform Arduino.

## Usage
- `arduino --verify jalan/menuju/berkas.ino` : bangun piranti lunak dari suatu berkas (sketsa) kode sumber
- `arduino --upload jalan/menuju/berkas.ino` : bangun dan pasang piranti lunak menuju perangkat Arduino
- `arduino --board arduino:avr:nano:cpu=atmega328p --port /dev/ttyACM0 --upload jalan/menuju/berkas.ino` : bangun dan pasang piranti lunak menuju suatu perangkat Arduino Nano dengan prosesor Atmega328p yang terhubung dalam port `/dev/ttyACM0`
- `arduino --pref nama=nilai` : atur `nilai` untuk suatu jenis preferensi/konfigurasi berdasarkan `nama` atau kata kunci
- `arduino --pref build.path=jalan/menuju/direktori_hasil_pembangunan --verify jalan/menuju/bekas.ino` : bangun piranti lunak, kemudian simpan menuju suatu direktori hasil pembangunan, dan gunakan kembali hasil-hasil sebelumnya di dalam direktori tersebut
- `arduino --save-prefs` : simpan segala perubahan pada preferensi/konfigurasi menuju berkas `preferences.txt`
- `arduino --install-boards "arduino:sam"` : pasang piranti pendukung pengembangan untuk perangkat Arduino berbasis SAM (seperti Arduino Due)
- `arduino --install-library "Bridge:1.0.0,Servo:1.2.0"` : pasang pustaka piranti lunak (library) untuk Bridge dan Servo

## Links
- More information: https://github.com/arduino/Arduino/blob/master/build/shared/manpage.adoc
