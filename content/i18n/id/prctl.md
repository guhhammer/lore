# prctl

Ambil atau atur sumber daya dari proses, tugas dan projek yang berjalan.

## Usage
- `prctl pid` : periksa batas dan perizinan proses
- `prctl -P pid` : periksa batas dan perizinan proses dalam format yang dapat diurai mesin
- `prctl -n process.max-file-descriptor pid` : ambil batas spesifik dari sebuah proses yang berjalan

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
