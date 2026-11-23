# yay

Yet Another Yogurt: bangun dan pasang paket piranti lunak dari Arch User Repository.

## Usage
- `yay nama_paket|kata_pencarian` : cari dan pasang kumpulan paket dari kumpulan repositori dan AUR secara interaktif
- `yay` : padankan daftar paket dan perbarui seluruh paket dari kumpulan repositori dan AUR
- `yay -S paket --noconfirm` : pasang baru suatu paket dari kumpulan repositori dan AUR dan tanpa perlu mengonfirmasi transaksi langkah pemasangan
- `yay -Rns paket` : hapus suatu paket terpasang beserta seluruh paket ketergantungannya dan berkas konfigurasinya
- `yay -Ss kata_kunci` : cari paket berdasarkan kata kunci tertentu dari kumpulan repositori dan AUR
- `yay -Yc` : hapus para paket yatim yang terpasang (yang sebelumnya terpasang sebagai ketergantungan namun kini tak dibutuhkan oleh paket apapun)
- `yay -Scc` : bersihkan data cache `pacman` dan `yay` (berupa versi-versi paket lawas yang ditahan untuk proses pemulihan dengan rollback dan downgrade)
- `yay -Ps` : tampilkan statistik pemasangan paket beserta kesehatan sistem operasi

## Links
- More information: https://github.com/Jguer/yay#first-use
