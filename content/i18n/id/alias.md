# alias

Buat alias perintah -- kata-kata yang digantikan oleh utasan perintah (command). Alias menjadi kadaluarsa sampai sesi shell saat ini berakhir, kecuali jika didefinisikan di file konfigurasi shell, misalnya `~/.bashrc`.

## Usage
- `alias` : tampilkan daftar seluruh alias yang terdaftar
- `alias kata="perintah"` : buat suatu alias generik
- `alias kata` : lihat perintah yang dirujuk oleh alias yang diberikan
- `unalias kata` : hapus suatu perintah alias
- `alias rm="rm --interactive"` : ubah perintah `rm` menjadi perintah interaktif
- `alias la="ls --all"` : buat perintah `la` menjadi pintasan untuk `ls --all`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
