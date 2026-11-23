# apt-mark

Alat untuk mengubah status pemasangan kumpulan paket.

## Usage
- `sudo apt-mark auto paket` : tandai suatu paket sebagai "terpasang secara otomatis" (misalnya, sebagai bagian ketergantungan / dependensi dari paket lain)
- `sudo apt-mark hold paket` : tahan suatu paket agar tidak diubah atau diperbarui ke dalam versi lainnya
- `sudo apt-mark unhold paket` : lepaskan penahanan suatu paket agar dapat diperbarui kembali
- `apt-mark showmanual` : tampilkan daftar paket yang terpasang secara manual
- `apt-mark showhold` : tampilkan daftar paket yang ditahan dari pembaruan peranti lunak

## Links
- More information: https://manned.org/apt-mark
