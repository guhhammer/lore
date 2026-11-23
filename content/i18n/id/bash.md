# bash

Bourne-Again SHell, suatu penerjemah baris perintah yang kompatibel dengan standar bahasa `sh`.

## Usage
- `bash` : jalankan sesi interaktif baru
- `bash --norc` : jalankan sesi interaktif baru tanpa memuat perintah konfigurasi awal
- `bash -c "echo 'perintah bash telah dijalankan'"` : jalankan kumpulan perintah secara spesifik
- `bash jalan/menuju/naskah.sh` : jalankan suatu naskah/skrip perintah
- `bash -x jalan/menuju/naskah.sh` : jalankan suatu naskah/skrip, dan tampilkan isi masing-masing perintah sebelum menjalankannya
- `bash -e jalan/menuju/naskah.sh` : jalankan suatu naskah/skrip dan segera berhenti jika terdapat galat/error perdana
- `echo "echo 'perintah bash telah dijalankan'" | bash` : jalankan kumpulan perintah dari `stdin`
- `bash -r` : jalankan sesi interaktif baru dengan pembatasan administratif

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Invoking-Bash
