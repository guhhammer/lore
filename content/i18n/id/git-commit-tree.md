# git commit-tree

Alat untuk membuat objek komit secara tingkat rendah (low-level).

## Usage
- `git commit-tree tree -m "pesan"` : buat objek komit baru dengan pesan tertentu
- `git commit-tree tree -F jalan/menuju/berkas` : buat objek komit dengan pesan yang disimpan dalam suatu berkas (gunakan `-` untuk membaca dari `stdin`)
- `git commit-tree tree -m "pesan" -S` : buat sebuah objek komit yang ditandatangani oleh kunci enkripsi GPG
- `git commit-tree tree -m "message" -p kode_hash_sha_atas_komit_induk` : buat sebuah objek komit dengan komit induk tertentu

## Links
- More information: https://git-scm.com/docs/git-commit-tree
