# nvim

Neovim, teks editor programmer berbasis Vim, menyediakan beberapa mode untuk manipulasi teks berbeda jenis. Tekan `<i>` masuk mode edit. `<Esc>` kembali ke mode normal, yang tidak mengizinkan penyisipan teks biasa.

## Usage
- `nvim jalan/menuju/berkas` : membuka berkas
- `<Esc><i>` : masuk ke mode pengeditan teks (insert mode)
- `<Esc><y><y>|<d><d>` : menyalin ("yank") atau memotong ("delete") baris saat ini (tempel itu dengan `<p>`)
- `<Esc><u>` : batalkan operasi terakhir
- `<Esc></>pattern_pencarian<Enter>` : mencari sebuah pattern pada berkas (tekan `<n>`/`<N>` untuk pergi ke kecocokan berikutnya/sebelumnya)
- `<Esc><:>%s/ekspresi_reguler/pengganti/g<Enter>` : melakukan penggantian ekspresi reguler pada seluruh berkas
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : menyimpan (write) berkas, dan keluar
- `<Esc><:>q!<Enter>` : keluar tanpa menyimpan

## Links
- More information: https://neovim.io/
