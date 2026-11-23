# git annex

Kelola file dengan Git, tanpa memeriksa isi kontennya. Saat file dianeksasi, kontennya dipindahkan ke penyimpanan key-value, dan symlink dibuat yang mengarah ke konten tersebut.

## Usage
- `git annex init` : inisialisasi sebuah repositori dengan Git annex
- `git annex add jalan/menuju/berkas_atau_direktori` : tambahkan file ke dalam repositori
- `git annex status jalan/menuju/berkas_atau_direktori` : tampilkan status file atau direktori saat ini
- `git annex remote` : sinkronisasikan repositori lokal dengan sumber remote
- `git annex get jalan/menuju/berkas_atau_direktori` : dapatkan isi file atau direktori
- `git annex help` : tampilkan bantuan

## Links
- More information: https://git-annex.branchable.com/git-annex/
