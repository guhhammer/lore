# git checkout-index

Salin file dari indeks menuju direktori kerja saat ini.

## Usage
- `git checkout-index -a` : pulihkan berkas-berkas yang terhapus sejak komit terakhir
- `git checkout-index -a -f` : pulihkan berkas-berkas yang terhapus atau termodifikasi sejak komit terakhir
- `git checkout-index -a -f -n` : pulihkan berkas-berkas yang diubah sejak komit terakhir, mengabaikan berkas-berkas yang telah dihapus sebelumnya
- `git checkout-index -a -f --prefix jalan/menuju/direktori_ekspor/` : ekspor sebuah salinan pohon (tree) pada komit terakhir kepada suatu direktori (nama direktori pada `--prefix` perlu diakhiri dengan garis miring)

## Links
- More information: https://git-scm.com/docs/git-checkout-index
