# tlmgr key

Atur daftar kunci GPG untuk memverifikasi pangkalan data paket TeX Live.

## Usage
- `tlmgr key list` : tampilkan seluruh kunci untuk TeX Live
- `sudo tlmgr key add jalan/menuju/kunci.gpg` : tambahkan suatu kunci dari berkas
- `cat jalan/menuju/kunci.gpg | sudo tlmgr key add -` : tambahkan suatu kunci dari `stdin`
- `sudo tlmgr key remove id_kunci` : hapus suatu kunci menurut nomor induknya (ID)

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#key
