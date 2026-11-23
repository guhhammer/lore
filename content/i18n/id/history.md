# history

Tampilkan riwayat penugasan baris perintah (command-line).

## Usage
- `history` : tampilkan riwayat penugasan baris perintah beserta angka baris
- `history 20` : tampilkan 20 perintah tugas terakhir (di Zsh perintah ini menampilkan semua perintah-perintah sejak dari baris ke-20)
- `history -d|f|i|E` : tampilkan riwayat dengan format tanggal dan waktu tertentu (hanya tersedia dalam Zsh)
- `history -c` : hapus seluruh riwayat perintah penugasan (hanya untuk sesi shell Bash saat ini)
- `history -w` : tulis ulang berkas dengan riwayat sesi shell Bash saat ini (seringkali dikombinasikan dengan `history -c` untuk menghapus riwayat)
- `history -d offset` : hapus entri riwayat pada offset tertentu
- `history -s perintah` : simpan sebuah baris perintah sebagai entri riwayat baru tanpa menjalankannya
- `<Space>perintah` : jalankan sebuah perintah tanpa memasukkan ke dalam riwayat, dengan membubuhkan karakter spasi pada awal baris

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-history
