# diff

Bandingkan isi berkas dan direktori.

## Usage
- `diff berkas_lawas berkas_baru` : bandingkan isi berkas-berkas (tampilkan daftar isi yang diubah dari `berkas_lawas` sehingga membentuk `berkas_baru`)
- `diff -w berkas_lawas berkas_baru` : bandingkan isi berkas-berkas, tanpa menghiraukan karakter spasi
- `diff -y berkas_lawas berkas_baru` : bandingkan isi berkas-berkas, dan tampilkan perbedaannya secara samping-menyamping
- `diff -u berkas_lawas berkas_baru` : bandingkan isi berkas-berkas, dan tampilkan perbedaannya dalam format menyatu (sebagaimana digunakan dalam `git diff`)
- `diff -r direktori_lawas direktori_baru` : bandingkan direktori-direktori secara rekursif (tampilkan nama-nama berkas/direktori beserta segala perubahan atas isi berkas-berkas)
- `diff -r -q direktori_lawas direktori_baru` : bandingkan direktori-direktori, dan hanya tampilkan nama berkas yang berbeda isi
- `diff -a -u -N berkas_lawas berkas_baru >diff.patch` : buat suatu berkas patch untuk Git dari perbedaan atas kedua berkas teks, dengan memperlakukan berkas yang tidak tersedia sebagai kosong
- `diff -d --color=always berkas_lawas berkas_baru` : bandingkan berkas-berkas, menampilkan luaran program secara berwarna dan upayakan untuk mencari perubahan-perubahan secara minimal

## Links
- More information: https://manned.org/diff
