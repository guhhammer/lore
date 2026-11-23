# git check-ignore

Analisa kumpulan berkas yang diabaikan/dikecualikan oleh Git (didefinisikan dalam `.gitignore`).

## Usage
- `git check-ignore jalan/menuju/berkas_atau_direktori` : cek apakah suatu berkas atau direktori telah diabaikan
- `git check-ignore jalan/menuju/berkas_atau_direktori1 jalan/menuju/berkas_atau_direktori2 ...` : cek apakah lebih dari satu berkas atau direktori telah diabaikan
- `git <jalan/menuju/berkas_daftar check-ignore --stdin` : cek pengecualian berkas dan direktori menggunakan daftar yang didefinisikan dalam `stdin`
- `git check-ignore --no-index jalan/menuju/berkas_atau_direktori1 jalan/menuju/berkas_atau_direktori2 ...` : jangan cek index Git (biasanya dipakai untuk mengetahui mengapa terdapat jalur yang tetap dilacak Git dan tak diabaikan)
- `git check-ignore -v jalan/menuju/berkas_atau_direktori1 jalan/menuju/berkas_atau_direktori2 ...` : tampilkan informasi pola pengecualian `.gitignore` yang dipakai untuk mengecualikan setiap jalur

## Links
- More information: https://git-scm.com/docs/git-check-ignore
