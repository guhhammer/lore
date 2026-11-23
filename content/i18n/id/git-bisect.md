# git bisect

Lakukan strategi pencarian/pembelahan biner untuk mencari komit yang menyebabkan masalah/bug. Git akan secara otomatis melompat bolak-balik dalam grafik komit untuk semakin mempersempit kandidat komit yang bermasalah.

## Usage
- `git bisect start komit_buruk komit_baik` : jalankan sesi pembelahan biner pada suatu rentang komit antara komit buruk/bermasalah dan komit (biasanya terdahulu) yang diketahui tak bermasalah
- `git bisect good|bad` : untuk setiap komit yang dipilih oleh `git bisect`, tandai komit tersebut sebagai baik (good) atau buruk (bad) setelah mencobanya
- `git bisect reset` : setelah `git bisect` berhasil menemukan komit yang bermasalah, akhiri sesi pembelahan dan kembali kepada cabang sebelumnya
- `git bisect skip` : lewati pengecekan suatu komit saat proses pembelahan berlangsung (misal: karena terdapat masalah yang disebabkan oleh faktor lain)
- `git bisect start komit_buruk komit_baik -- jalan/menuju/berkas_atau_direktori` : jalankan sesi pembelahan baru dengan hanya menghiraukan kumpulan komit yang merubah isi suatu berkas atau direktori secara spesifik
- `git bisect run jalan/menuju/skrip_penguji argumen_skrip_opsional` : lakukan prosesi pembelahan secara otomatis menggunakan suatu skrip penguji yang mengeluarkan kode `exit` 0 untuk hasil baik/"good" dan non-0 untuk buruk/"bad"
- `git bisect log` : tampilkan log tentang kemajuan proses pembelahan saat ini
- `git bisect visualize` : tampilkan sisa daftar komit kandidat untuk diperiksa dalam proses pembelahan ini

## Links
- More information: https://git-scm.com/docs/git-bisect
