# vim

Vim (Vi IMproved), suatu aplikasi pengolah teks berbasis baris perintah, yang menyediakan beberapa mode untuk berbagai jenis proses manipulasi teks. Menekan `<i>` dalam mode normal akan memasuki mode penyisipan (insert). Menekan `<Esc>` akan kembali ke mode normal, yang memungkinkan penggunaan perintah Vim.

## Usage
- `vim jalan/menuju/berkas` : buka suatu berkas
- `vim +nomor_baris jalan/menuju/berkas` : buka suatu berkas pada nomor baris teks tertentu
- `<:>help<Enter>` : lihat manual bantuan untuk Vim
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : simpan dan keluar dari sesi pengolahan teks saat ini
- `<Esc><u>` : masuk ke mode normal dan batalkan operasi terakhir
- `</>pola_pencarian<Enter>` : cari pola dalam berkas (tekan `<n>`/`<N>` untuk menuju ke kecocokan berikutnya/sebelumnya)
- `<:>%s/ekspresi_reguler/teks_pengganti/g<Enter>` : lakukan substitusi ekspresi reguler di seluruh berkas
- `<:>set nu<Enter>` : tampilkan nomor baris

## Links
- More information: https://www.vim.org/
