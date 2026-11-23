# pacman-remove

Kegunaan manajer paket Arch Linux.

## Usage
- `sudo pacman -Rs paket` : hapus paket beserta dependensinya
- `sudo pacman -Rsn paket` : hapus paket beserta dependensi dan file konfigurasi-nya
- `sudo pacman -R --noconfirm paket` : hapus tanpa konfirmasi
- `sudo pacman -Rsn $(pacman -Qdtq)` : hapus paket yatim piatu (diinstal sebagai dependensi namun tidak dibutuhkan oleh paket apa pun)
- `sudo pacman -Rc paket` : hapus paket dan semua paket yang bergantung pada paket tersebut
- `pacman -Rp paket` : tampilkan daftar paket yang akan terpengaruh (tidak menghapus paket apa pun)
- `pacman -Rh` : tampilkan bantuan untuk subperintah ini

## Links
- More information: https://manned.org/pacman.8
