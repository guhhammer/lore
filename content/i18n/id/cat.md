# cat

Cetak dan menggabungkan berkas.

## Usage
- `cat jalan/menuju/berkas` : cetak konten berkas menuju `stdout`
- `cat jalan/menuju/berkas1 jalan/menuju/berkas2 ... >jalan/menuju/berkas_tujuan` : gabungkan konten beberapa berkas ke berkas tujuan
- `cat jalan/menuju/berkas1 jalan/menuju/berkas2 ... >>jalan/menuju/berkas_tujuan` : tambahkan konten beberapa berkas ke berkas tujuan
- `cat -u /dev/tty12 >/dev/tty13` : salin isi suatu file menuju file tujuan tanpa proses buffering
- `cat - >jalan/menuju/berkas` : tulis isi `stdin` menuju suatu file

## Links
- More information: https://manned.org/cat.1posix
