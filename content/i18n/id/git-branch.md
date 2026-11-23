# git branch

Perintah Git utama untuk bekerja dengan cabang (branch).

## Usage
- `git branch -a` : tampilkan daftar semua cabang (lokal dan remote; cabang saat ini ditandai oleh `*`)
- `git branch -a --contains hash_komit` : tampilkan daftar semua cabang yang memiliki komit Git tertentu di dalam riwayat
- `git branch --show-current` : tampilkan nama cabang saat ini
- `git branch nama_cabang` : buat cabang baru berdasarkan komit saat ini
- `git branch nama_cabang hash_komit` : buat cabang baru berdasarkan komit tertentu
- `git branch -m nama_cabang_lama nama_cabang_baru` : ubah nama cabang (harus bukan cabang saat ini untuk melakukannya)
- `git branch -d nama_cabang` : hapus cabang lokal (harus bukan cabang saat ini untuk melakukannya)
- `git push nama_remote -d nama_cabang_remote` : hapus cabang remote

## Links
- More information: https://git-scm.com/docs/git-branch
