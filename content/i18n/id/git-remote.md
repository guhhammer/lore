# git remote

Kelola kumpulan repositori yang dilacak/diikuti dari sumber jarak jauh ("remotes").

## Usage
- `git remote -v` : tampilkan daftar remote, namanya dan URL
- `git remote show nama_remote` : tampilkan informasi tentang suatu remote
- `git remote add nama_remote url_remote` : tambahkan suatu remote untuk diikuti pada repositori saat ini
- `git remote set-url nama_remote url_baru` : ubah alamat URL dari remote (gunakan `--add` untuk tetap menyimpan URL lama)
- `git remote get-url nama_remote` : tampilkan alamat URL dari suatu remote
- `git remote remove nama_remote` : hapus remote dari daftar remote yang dilacak pada repositori saat ini
- `git remote rename nama_lama nama_baru` : ubah nama remote untuk dikelola dalam repositori saat ini

## Links
- More information: https://git-scm.com/docs/git-remote
