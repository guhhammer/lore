# dnf

Manajer paket untuk distribusi Linux RHEL, Fedora, dan CentOS (pengganti yum).

## Usage
- `sudo dnf upgrade` : perbarui seluruh paket yang terpasang ke versi terbaru
- `dnf search kata_kunci1 kata_kunci2 ...` : cari paket yang tersedia dengan kata-kata kunci tertentu
- `dnf info paket` : tampilkan informasi tentang suatu paket
- `sudo dnf install paket1 paket2 ...` : pasang kumpulan paket (gunakan `-y` jawab untuk ya semua pertanyaan)
- `sudo dnf remove paket1 paket2 ...` : hapus kumpulan paket
- `dnf list --installed` : tampilkan daftar semua paket yang telah terpasang
- `dnf provides perintah` : temukan paket mana yang menyediakan perintah tertentu
- `dnf history` : lihat informasi riwayat penugasan `dnf`

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
