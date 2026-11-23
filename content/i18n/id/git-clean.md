# git clean

Hapus berkas-berkas yang tak dilacak oleh Git pada pohon direktori kerja saat ini.

## Usage
- `git clean -i` : hapus menggunakan mode interaktif
- `git clean -n` : tampilkan kumpulan berkas yang akan dihapus tanpa menghapusnya
- `git clean -f` : hapus berkas-berkas secara paksa
- `git clean -f -d` : hapus kumpulan direktori secara paksa
- `git clean -f -- jalan/menuju/direktori '*.ext'` : hanya hapus berkas-berkas tak dilacak dengan alamat berkas spesifik atau pola (`glob`) nama yang sesuai dengan yang ditentukan
- `git clean -f -e '*.ext' -e jalan/menuju/direktori/` : hapus berkas-berkas tak dilacak kecuali dalam alamat atau pola nama yang ditentukan
- `git clean -f -x` : hapus berkas-berkas yang tak dilacak, termasuk berkas yang dikecualikan (menurut daftar `.gitignore` dan `.git/info/exclude`)

## Links
- More information: https://git-scm.com/docs/git-clean
