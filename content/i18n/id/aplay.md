# aplay

Pemutar suara untuk driver kartu suara ALSA.

## Usage
- `aplay jalan/menuju/berkas` : dengarkan file audio tertentu (sampling rate, bit depth, dll. akan ditentukan secara otomatis berdasarkan format file)
- `aplay -d 10 -r 2500 jalan/menuju/berkas` : dengarkan file audio 10 detik pertama dari file tertentu pada 2500 Hz
- `aplay -c 1 -t raw -r 22050 -f mu_law jalan/menuju/berkas` : dengarkan file raw sebagai file `.au` dengan 22050 Hz, mono, 8-bit, Mu-Law
- `aplay -l` : tampilkan daftar perangkat audio yang tersedia

## Links
- More information: https://manned.org/aplay
