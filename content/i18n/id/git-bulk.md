# git bulk

Lakukan operasi yang sama dalam lebih dari satu repositori Git. Bagian dari `git-extras`.

## Usage
- `git bulk --addcurrent nama_workspace` : daftarkan direktori saat ini sebagai tempat kerja (workspace)
- `git bulk --addworkspace nama_workspace /jalan/menuju/repositori` : masukkan tempat kerja saat ini ke dalam daftar direktori yang akan diubah
- `git bulk --addworkspace nama_workspace /jalan/menuju/direktori_induk --from lokasi_repositori_remote` : gandakan suatu repositori ke dalam direktori induk tertentu, kemudian masukkan repositori baru tersebut sebagai tempat kerja
- `git bulk --addworkspace nama_workspace /jalan/menuju/direktori_induk --from /jalan/menuju/berkas` : gandakan lebih dari satu repositori ke dalam direktori induk tertentu (menurut berkas daftar lokasi remote yang dipisah dengan barisan baru), kemudian masukkan sebagai tempat kerja
- `git bulk --listall` : tampilkan daftar seluruh tempat kerja yang terdaftar
- `git bulk perintah argumen-argumen_perintah` : jalankan sebuah perintah Git pada kumpulan repositori yang dikelola oleh tempat kerja saat ini
- `git bulk --removeworkspace nama_workspace` : hapus suatu tempat dari daftar tempat kerja (hal ini tidak akan menghilangkan seluruh isi direktori yang direferensikan sebagai tempat kerja)
- `git bulk --purge` : hapus seluruh tempat dari daftar tempat kerja

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-bulk
