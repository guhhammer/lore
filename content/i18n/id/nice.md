# nice

Jalankan suatu program dengan tingkatan prioritas penjadwalan secara kustom. Nilai prioritas (niceness) memili rentang antara -20 (prioritas terbesar) menuju 19 (terkecil). Catatan: Beberapa sistem penjadwalan modern mengabaikan nilai ini atau efeknya dalam kumpulan autogroup.

## Usage
- `nice` : tampilkan nilai niceness saat ini
- `nice nice` : naikkan nilai niceness dengan 10
- `nice -nilai_niceness perintah` : jalankan suatu program dengan prioritas rendah
- `sudo nice --nilai_niceness perintah` : jalankan suatu program dengan prioritas tinggi
- `nice -n -20..19 perintah` : tentukan prioritas secara eksplisit

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/nice-invocation.html
