# tlmgr backup

Atur pengaturan pencadangan kumpulan paket TeX Live. Direktori pencadangan default ditentukan sebagaimana diatur dalam opsi `backupdir`, dan dapat didapatkan menggunakan `tlmgr option`.

## Usage
- `tlmgr backup paket1 paket2 ...` : buat suatu pencadangan atas satu atau lebih banyak paket
- `tlmgr backup --all` : buat suatu pencadangan terhadap seluruh paket
- `tlmgr backup paket --backupdir jalan/menuju/direktori_pencadangan` : buat suatu pencadangan menuju suatu direktori kustom
- `tlmgr backup clean paket1 paket2 ...` : hapus hasil-hasil pencadangan terhadap satu atau lebih banyak paket
- `tlmgr backup clean --all` : hapus seluruh hasil pencadangan terhadap seluruh paket

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#backup
