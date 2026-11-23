# fish

The Friendly Interactive SHell, suatu penerjemah baris perintah yang dirancang secara ramah pengguna.

## Usage
- `fish` : jalankan sesi interaktif baru
- `fish -N` : jalankan sesi interaktif baru tanpa memuat perintah konfigurasi awal
- `fish -c "echo 'fish is executed'"` : jalankan kumpulan perintah secara spesifik
- `fish jalan/menuju/naskah.fish` : jalankan suatu naskah/skrip perintah
- `fish -n jalan/menuju/naskah.fish` : periksa kesalahan sintaks pada suatu naskah
- `echo "echo 'perintah fish telah dijalankan'" | fish` : jalankan kumpulan perintah dari `stdin`
- `fish -P` : jalankan sesi interaktif baru dalam mode Privat, di mana shell tidak akan mengakses riwayat perintah lama maupun memasukkan riwayat perintah baru
- `set -U -x nama_variabel nilai_variable` : tentukan dan ekspor nilai-nilai variabel lingkungan yang disimpan untuk setiap perjalanan sesi baru (builtin environment variable)

## Links
- More information: https://fishshell.com/docs/current/cmds/fish.html
