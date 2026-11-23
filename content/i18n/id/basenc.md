# basenc

Tulis ulang isi suatu berkas atau `stdin` dalam format biner dari/menuju format penulisan enkoding tertentu, menuju `stdout`.

## Usage
- `basenc --base64 jalan/menuju/berkas` : tulis ulang isi biner suatu berkas dalam format Base64
- `basenc -d --base64 jalan/menuju/berkas` : tulis ulang isi suatu berkas dari format Base64 menuju format asli/biner
- `perintah | basenc --base32 -w 42` : tulis ulang isi biner `stdin` menuju format Base32 dengan panjang baris 42 karakter
- `perintah | basenc --base32` : tulis ulang isi biner `stdin` menuju format Base32
