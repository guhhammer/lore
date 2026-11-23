# git push

Dorong kumpulan komit menuju suatu repositori jarak jauh (remote).

## Usage
- `git push` : kirim perubahan lokal dari cabang (branch) saat ini menuju cabang yang sepadan pada repositori tujuan
- `git push nama_remote cabang_lokal` : kirim perubahan dari cabang lokal yang ditentukan menuju cabang yang sepadan pada repositori tujuan
- `git push -u nama_remote cabang_lokal` : kirim perubahan dari cabang lokal yang ditentukan menuju cabang sepadan pada repositori tujuan, dan simpan remote sebagai target operasi dorong (push) dan tarik (pull) bagi cabang lokal tersebut
- `git push nama_remote cabang_lokal:cabang_remote` : kirim perubahan dari suatu cabang lokal menuju suatu cabang remote secara spesifik
- `git push --all nama-remote` : kirim perubahan dari setiap cabang lokal menuju cabang-cabang sepadan dalam repositori tujuan
- `git push nama_remote -d cabang_remote` : hapus suatu cabang dalam suatu repositori remote
- `git push --prune nama_remote` : hapus cabang-cabang remote yang tidak memiliki padanan pada repositori lokal
- `git push --tags` : publikasikan kumpulan tag komit yang belum dipublikasikan dalam repositori remote

## Links
- More information: https://git-scm.com/docs/git-push
