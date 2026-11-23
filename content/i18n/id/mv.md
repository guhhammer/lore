# mv

Pindahkan atau namai-ulang berkas dan direktori.

## Usage
- `mv jalan/menuju/sumber jalan/menuju/tujuan` : pindahkan berkas ke lokasi yang baru
- `mv jalan/menuju/sumber jalan/menuju/direktori` : pindahkan berkas atau direktori ke dalam direktori lain yang telah ada
- `mv jalan/menuju/sumber1 jalan/menuju/sumber2 ... jalan/menuju/direktori` : pindahkan berkas majemuk ke dalam direktori lain yang telah ada, dengan menyimpan nama masing-masing file secara utuh
- `mv -f jalan/menuju/sumber jalan/menuju/tujuan` : pindahkan tanpa meminta konfirmasi sebelum menimpa file yang sudah ada
- `mv -i jalan/menuju/sumber jalan/menuju/tujuan` : minta konfirmasi sebelum menimpa berkas yang sudah ada, tanpa memerhatikan hak akses hedua file tersebut
- `mv -n jalan/menuju/sumber jalan/menuju/tujuan` : jangan menimpa file yang sudah ada di direktori tujuan
- `mv -v jalan/menuju/sumber jalan/menuju/tujuan` : pindahkan berkas dalam mode verbose, tampilkan berkas-berkas yang dipindahkan
- `find /var/log -type f -name '*.log' -print0 | xargs -0 mv -t jalan/menuju/direktori_tujuan` : tetapkan direktori tujuan target agar Anda dapat menggunakan alat atau perintah eksternal untuk mengelola kumpulan berkas yang dapat dipindahkan

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
