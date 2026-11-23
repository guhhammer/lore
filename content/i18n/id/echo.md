# echo

Cetak ulang argumen-argumen yang dimasukkan ke dalam layar perangkat.

## Usage
- `echo "Halo Dunia"` : cetak sebuah pesan teks. Catatan: penggunaan tanda petik bersifat opsional
- `echo "Variabel path saya adalah $PATH"` : cetak sebuah pesan bersama suatu variabel lingkungan (environment variable)
- `echo -n "Halo Dunia"` : cetak sebuah pesan tanpa mencetak baris teks baru (tanpa newline)
- `echo "Halo Dunia" >>berkas.txt` : tambahkan isi pesan ke dalam suatu berkas teks
- `echo -e "Kolom 1\tKolom 2"` : aktifkan fitur interpretasi penggunaan tanda garis miring terbalik sebagai penanda karakter khusus
- `echo $?` : cetak status keluar dari perintah terakhir yang dieksekusi (Catatan: Dalam Windows Command Prompt dan PowerShell, perintah yang setara adalah `echo %errorlevel%` dan `$lastexitcode`)
- `echo "Halo Dunia" | program` : teruskan suatu pesan teks menuju program lainnya menuju `stdin`

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
