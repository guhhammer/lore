# dash

Debian Almquist Shell, suatu implementasi `sh` berstandar POSIX (tidak kompatibel dengan Bash).

## Usage
- `dash` : jalankan sesi interaktif baru
- `dash -c "echo 'perintah dash telah dijalankan'"` : jalankan kumpulan perintah secara spesifik
- `dash jalan/menuju/naskah.sh` : jalankan suatu naskah/skrip perintah
- `dash -n jalan/menuju/naskah.sh` : jalankan sesi interaktif baru dengan mengeluarkan kembali isi perintah masukan sebelum menjalankannya
- `dash -x jalan/menuju/naskah.sh` : jalankan suatu naskah/skrip, dan tampilkan isi masing-masing perintah sebelum menjalankannya
- `dash -e jalan/menuju/naskah.sh` : jalankan suatu naskah/skrip dan segera berhenti jika terdapat galat/error perdana
- `echo "echo 'perintah dash telah dijalankan'" | dash` : jalankan kumpulan perintah dari `stdin`

## Links
- More information: https://manned.org/dash
