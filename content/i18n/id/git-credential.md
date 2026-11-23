# git credential

Terima dan simpan kredensial pengguna.

## Usage
- `echo "url=http://example.com" | git credential fill` : tampilkan informasi suatu kredensial, termasuk username dan kata sandi dari berkas-berkas konfigurasi
- `echo "url=http://example.com" | git credential approve` : kirim informasi kredensial menuju seluruh piranti pembantu (credential helper) yang disetel untuk disimpan dan digunakan pada lain waktu
- `echo "url=http://example.com" | git credential reject` : hapus suatu informasi kredensial dari penyimpanan seluruh piranti pembantu

## Links
- More information: https://git-scm.com/docs/git-credential
