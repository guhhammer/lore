# a2ping

Ubah berkas gambar menjadi EPS atau PDF.

## Usage
- `a2ping jalan/menuju/gambar.ext jalan/menuju/output.pdf` : ubah sebuah gambar menjadi PDF (Catatan: Nama berkas output bersifat opsional)
- `a2ping --nocompress none|zip|best|flate jalan/menuju/berkas` : kompres dokumen EPS atau PDF menggunakan metode tertentu
- `a2ping --nohires jalan/menuju/berkas` : pindai HiResBoundingBox jika ditemukan (akan dipindai secara default)
- `a2ping --below jalan/menuju/berkas` : izinkan konten halaman berada melewati batas bawah dan kiri (tidak diizinkan secara default)
- `a2ping --gsextra argumen_tambahan_gs jalan/menuju/berkas` : berikan opsi/argumen tambahan untuk `gs`
- `a2ping --extra arguments jalan/menuju/berkas` : berikan opsi/argumen tambahan untuk program lainnya (seperti `pdftops`)
- `a2ping -h` : tampilkan bantuan

## Links
- More information: https://manned.org/a2ping
