# git bug

Manajer laporan masalah/bug yang menggunakan penyimpanan git, sehingga tidak memengaruhi susunan berkas dalam direktori proyek Anda. Anda dapat memasukkan laporan melalui sumber/hulu jauh (remote) yang sama untuk berinteraksi dengan laporan dan pengguna lainnya seperti mengatur komit dan cabang.

## Usage
- `git bug user create` : buat identitas/pengguna baru
- `git bug add` : buat laporan masalah/bug baru
- `git bug push` : kumpulkan laporan-laporan baru menuju sumber/hulu jarak jauh
- `git bug pull` : dapatkan pembaruan atas daftar masalah dari sumber/hulu jarak jauh
- `git bug ls` : lihat daftar masalah/bug yang sebelumnya telah dilaporkan
- `git bug ls "status:open sort:edit"` : saring (filter) dan urutkan (sort) laporan menggunakan kata kunci permintaan tertentu
- `git bug ls "kata_kunci" baz` : cari laporan menurut kata kunci teks

## Links
- More information: https://github.com/git-bug/git-bug/blob/master/doc/md/git-bug.md
