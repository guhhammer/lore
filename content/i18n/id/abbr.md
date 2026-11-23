# abbr

Kelola singkatan untuk fish shell. Kata yang didefinisikan pengguna diganti dengan frasa yang lebih panjang setelah dimasukkan.

## Usage
- `abbr -a abbreviation_name perintah command_arguments` : tambahkan singkatan baru
- `abbr --rename old_name new_name` : ganti nama singkatan yang sudah ada
- `abbr -e abbreviation_name` : hapus singkatan yang sudah ada
- `ssh host_name abbr -s | source` : impor singkatan yang didefinisikan pada host lain melalui SSH

## Links
- More information: https://fishshell.com/docs/current/cmds/abbr.html
