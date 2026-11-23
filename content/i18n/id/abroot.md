# abroot

Utilitas yang menyediakan imutabilitas dan atomisitas penuh melalui transaksi antara 2 status partisi root (A<=>B). Pembaruan dilakukan menggunakan image OCI, untuk memastikan sistem selalu dalam keadaan konsisten.

## Usage
- `sudo abroot pkg add paket` : tambahkan paket ke image lokal (Catatan: Setelah menjalankan perintah ini, Anda perlu menerapkan perubahan tersebut)
- `sudo abroot pkg remove paket` : hapus paket dari image lokal (Catatan: Setelah menjalankan perintah ini, Anda perlu menerapkan perubahan tersebut)
- `sudo abroot pkg list` : tampilkan daftar paket dalam image lokal
- `sudo abroot pkg apply` : terapkan perubahan dalam image lokal (Catatan: Anda perlu me-reboot sistem agar perubahan ini diterapkan)
- `sudo abroot rollback` : rollback sistem ke keadaan sebelumnya
- `sudo abroot kargs edit|show` : edit/Lihat parameter kernel
- `sudo abroot status` : tampilkan status
- `abroot -h` : tampilkan bantuan

## Links
- More information: https://docs.vanillaos.org/docs/en/abroot-manpage
