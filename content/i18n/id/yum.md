# yum

Utilitas manajemen paket untuk RHEL, Fedora, dan CentOS (untuk versi-versi yang lebih lama).

## Usage
- `yum install nama_paket` : pasang suatu paket
- `yum -y install nama_paket` : pasang paket dengan mengasumsikan jawaban ya untuk semua pertanyaan (juga berfungsi dengan perintah pembaruan, sangat berguna untuk pembaruan otomatis)
- `yum provides perintah` : cari sebuah paket yang menyediakan suatu perintah tertentu
- `yum remove paket` : hapus paket yang terpasang sebelumnya
- `yum check-update` : tampilkan pembaruan yang tersedia untuk paket yang terpasang
- `yum upgrade` : perbarui seluruh paket yang terpasang ke versi terbaru

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
