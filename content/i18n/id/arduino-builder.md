# arduino-builder

Bangun program dari kode sumber piranti lunak (sketsa) Arduino. PERINGATAN DEPREKASI: Alat ini sedang dihapus demi penggunaan perintah `arduino` yang baru.

## Usage
- `arduino-builder -compile jalan/menuju/sketsa.ino` : bangun program dari suatu berkas (sketsa) kode sumber piranti lunak
- `arduino-builder -debug-level 1..10` : tentukan tingkat penampilan informasi awakutu (nilai bawaan: 5)
- `arduino-builder -build-path jalan/menuju/direktori_hasil_pembangunan` : tentukan direktori untuk menampung hasil pembangunan
- `arduino-builder -build-options-file jalan/menuju/build.options.json` : gunakan konfigurasi yang didefinisikan di dalam suatu berkas, daripada mendefinisikan parameter perintah seperti `-hardware` dan `-tools` berulang kali
- `arduino-builder -verbose true` : gunakan mode verbose, tampilkan proses pembangunan secara rinci

## Links
- More information: https://github.com/arduino/arduino-builder
