# dpkg

Manajer paket Debian. Beberapa subperintah seperti `dpkg deb` memiliki dokumentasi penggunaannya sendiri.

## Usage
- `sudo dpkg -i jalan/menuju/berkas.deb` : pasang paket dari sebuah berkas DEB
- `sudo dpkg -r paket` : hapus pemasangan sebuah paket
- `dpkg -l pola` : tampilkan daftar paket terinstal
- `dpkg -L paket` : tampilkan rincian isi suatu paket
- `dpkg -c jalan/menuju/berkas.deb` : tampilkan rincian isi berkas sebuah paket lokal
- `dpkg -S jalan/menuju/berkas` : cari tahu paket yang memiliki sebuah berkas
- `sudo dpkg -P paket` : hapus pemasangan paket beserta berkas konfigurasi yang dibentuk oleh paket tersebut

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
