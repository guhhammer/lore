# pacman-sync

Kegunaan manajer paket Arch Linux.

## Usage
- `sudo pacman -S paket` : pasang suatu paket
- `sudo pacman -Syu` : sinkronkan dan perbarui semua paket (tambahkan `--downloadonly` untuk unduh paket dan tidak memperbarui-nya)
- `sudo pacman -Syu --noconfirm paket` : perbarui semua paket dan instal paket baru tanpa konfirmasi
- `pacman -Ss "pola_pencarian"` : cari paket dalam database berdasarkan `regex` atau kata kunci
- `pacman -Si paket` : tampilkan informasi suatu paket
- `sudo pacman -Syu --overwrite jalan/menuju/berkas` : timpa file yang bentrok selama pembaruan paket
- `sudo pacman -Sc` : hapus kumpulan paket dan repositori yang tidak terpakai dari cache (gunakan opsi flag `Scc` untuk hapus seluruh paket)
- `sudo pacman -S paket=versi` : tentukan versi paket yang hendak dipasang

## Links
- More information: https://manned.org/pacman.8
