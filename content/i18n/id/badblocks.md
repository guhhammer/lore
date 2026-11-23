# badblocks

Cari blok-blok memori yang bermasalah dalam suatu piranti penyimpanan. Penggunaan beberapa fungsi dalam badblocks dapat mengakibatkan perubahan destruktif, seperti menghapus seluruh data dalam suatu piranti penyimnpanan, termasuk tabel konfigurasi partisi.

## Usage
- `sudo badblocks /dev/sdX` : lakukan inspeksi terhadap suatu piranti penyimpanan menggunakan mode uji hanya-baca (read-only) yang non-destruktif
- `sudo badblocks -n /dev/sdX` : lakukan inspeksi terhadap suatu piranti yang tidak dimuat menggunakan mode uji baca-tulis yang non-destruktif
- `sudo badblocks -w /dev/sdX` : lakukan inspeksi terhadap suatu piranti yang tidak dimuat menggunakan mode uji baca (write) secara destruktif
- `sudo badblocks -svw /dev/sdX` : gunakan mode uji baca (write) destruktif dan tampilkan kemajuan proses secara bertele-tele
- `sudo badblocks -o jalan/menuju/berkas -w /dev/sdX` : dalam mode destruktif, simpan luaran (output) ke dalam suatu berkas
- `sudo badblocks -w -b 4096 -c 65536 /dev/sdX` : gunakan mode destruktif dalam kecepatan yang lebih baik menggunakan ukuran blok sebesar 4K dan jumlah blok (count) sebesar 64K

## Links
- More information: https://manned.org/badblocks
