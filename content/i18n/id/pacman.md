# pacman

Kegunaan manajer paket Arch Linux.

## Usage
- `sudo pacman -Syu` : sinkronkan dan perbarui semua paket
- `sudo pacman -S paket` : pasang suatu paket baru
- `sudo pacman -Rs paket` : hapus paket beserta dependensinya
- `pacman -F "nama_berkas"` : cari pangkalan data untuk nama-nama paket yang mengandung suatu berkas secara spesifik
- `pacman -Q` : tampilkan daftar paket dan versi yang diinstal
- `pacman -Qe` : tampilkan daftar paket dan versi yang diinstal secara eksplisit
- `pacman -Qtdq` : tampilkan daftar paket yatim piatu (diinstal sebagai dependensi namun tidak dibutuhkan oleh paket apa pun)
- `sudo pacman -Scc` : kosongkan seluruh cache pacman

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
