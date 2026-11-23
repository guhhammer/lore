# git cat-file

Dapatkan informasi konten atau jenis dan ukuran untuk objek repositori Git.

## Usage
- `git cat-file -s HEAD` : dapatkan ukuran (size) untuk komit terkini (HEAD), dalam hitungan bita/byte
- `git cat-file -t 8c442dc3` : dapatkan tipe yang direferensikan dalam suatu objek Git (seperti blob, tree, komit, atau tag)
- `git cat-file -p HEAD~2` : cetak isi objek Git yang diberikan berdasarkan jenisnya, dalam format yang mudah dibaca manusia

## Links
- More information: https://git-scm.com/docs/git-cat-file
