# ab

Alat penguji server HTTP Apache.

## Usage
- `ab -n 100 url` : jalankan 100 permintaan HTTP GET menuju alamat URL yang ditentukan
- `ab -n 100 -c 10 url` : jalankan 100 permintaan HTTP GET, dikelompokkan dalam masing-masing batch berisi 10, menuju alamat URL yang ditentukan
- `ab -n 100 -T application/json -p jalan/menuju/berkas.json url` : jalankan 100 perintaan HTTP POST menuju alamat URL, menggunakan data JSON yang dimuat dari file yang ditentukan
- `ab -k url` : gunakan opsi HTTP keep-Alive, yakni jalankan permintaan majemuk dalam satu sesi HTTP yang sama
- `ab -t 60 url` : alokasikan waktu maksimum (dalam hitungan detik) untuk mengujinya
- `ab -e jalan/menuju/berkas.csv` : simpan hasil pengujian menuju suatu berkas CSV

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
