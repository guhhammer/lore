# base64

Tulis ulang isi suatu berkas atau `stdin` dalam format biner dari/menuju format penulisan enkoding Base64, menuju `stdout`.

## Usage
- `base64 jalan/menuju/berkas` : tulis ulang isi biner suatu berkas dalam format Base64
- `base64 -w 0|76|... jalan/menuju/berkas` : bungkus luaran Base64 dalam panjang karakter yang tetap (nilai `0` akan menonaktifkan pembungkusan)
- `base64 -d jalan/menuju/berkas` : tulis ulang isi suatu berkas dari format Base64 menuju format asli/biner
- `perintah | base64` : tulis ulang isi biner `stdin` menuju format Base32
- `perintah | base64 -d` : tulis ulang isi Base64 `stdin` menuju format asli/biner

## Links
- More information: https://manned.org/base64
