# age

Alat pengenkripsi berkas yang sederhana, modern, dan aman.

## Usage
- `age -p -o jalan/menuju/berkas_terenkripsi jalan/menuju/berkas_tidak_terenkripsi` : buat sebuah berkas terenkripsi yang hanya dapat didekripsi menggunakan kata sandi
- `age -r kunci_publik -o jalan/menuju/berkas_terenkripsi jalan/menuju/berkas_tidak_terenkripsi` : buat berkas terenkripsi dengan kunci publik (public key) secara literal (ulangi argumen `--recipient` untuk memberikan kunci publik tambahan)
- `age -R path/to/berkas_daftar_penerima -o jalan/menuju/berkas_terenkripsi jalan/menuju/berkas_tidak_terenkripsi` : buat berkas terenkripsi bagi para penerima menurut kunci-kunci publik mereka yang disimpan di dalam suatu berkas (satu kunci per baris)
- `age -d -o path/to/berkas_bebas_enkripsi jalan/menuju/berkas_terenkripsi` : buka berkas terenkripsi dengan kata sandi
- `age -d -i path/to/berkas_kunci_privat -o path/to/berkas_bebas_enkripsi jalan/menuju/berkas_terenkripsi` : buka berkas terenkripsi dengan kunci privat

## Links
- More information: https://github.com/FiloSottile/age#usage
