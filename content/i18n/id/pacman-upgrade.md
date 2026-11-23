# pacman-upgrade

Kegunaan manajer paket Arch Linux.

## Usage
- `sudo pacman -U jalan/menuju/paket1.pkg.tar.zst jalan/menuju/paket2.pkg.tar.zst ...` : pasang suatu atau beberapa paket atau dari kumpulan berkas
- `sudo pacman -U --noconfirm jalan/menuju/paket.pkg.tar.zst` : pasang paket tanpa konfirmasi
- `sudo pacman -U --overwrite jalan/menuju/berkas jalan/menuju/paket.pkg.tar.zst` : timpa berkas-berkas sistem yang bentrok selama pemasangan paket
- `sudo pacman -Ud jalan/menuju/paket.pkg.tar.zst` : pasang paket, melewati pemeriksaan versi dependensi
- `pacman -Up jalan/menuju/paket.pkg.tar.zst` : tampilkan (print) daftar paket yang akan terpengaruh (tidak menginstal paket apa pun)
- `pacman -Uh` : tampilkan bantuan

## Links
- More information: https://manned.org/pacman.8
