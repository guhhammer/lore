# apt-file

Cari kumpulan berkas di dalam paket `apt`, termasuk yang belum dipasang.

## Usage
- `sudo apt update` : perbarui basis data metadata
- `apt-file find jalan/menuju/berkas` : cari paket yang berisi nama atau lokasi berkas tertentu
- `apt-file list paket` : tampilkan daftar konten dari sebuah paket
- `apt-file find -x ekspresi_reguler` : cari paket yang sesuai dengan `ekspresi_reguler`

## Links
- More information: https://manned.org/apt-file
