# base32

Tulis ulang isi suatu berkas atau `stdin` dalam format biner dari/menuju format penulisan enkoding Base32, menuju `stdout`.

## Usage
- `base32 jalan/menuju/berkas` : tulis ulang isi biner suatu berkas dalam format Base32
- `base32 -w 0|76|... jalan/menuju/berkas` : bungkus luaran Base32 dalam panjang karakter yang tetap (nilai `0` akan menonaktifkan pembungkusan)
- `base32 -d jalan/menuju/berkas` : tulis ulang isi suatu berkas dari format Base32 menuju format asli/biner
- `perintah | base32` : tulis ulang isi biner `stdin` menuju format Base32
- `perintah | base32 -d` : tulis ulang isi Base32 `stdin` menuju format asli/biner

## Links
- More information: https://manned.org/base32
