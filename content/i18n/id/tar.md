# tar

Alat pengarsip berkas. Sering digunakan bersamaan dengan alat kompresi tertentu, seperti `gzip` atau `bzip2`.

## Usage
- `tar cf jalan/menuju/target.tar jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : buat (create) suatu arsip dan simpan ke dalam suatu berkas (file)
- `tar czf jalan/menuju/target.tar.gz jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : buat (create) suatu arsip dengan tambahan kompresi gzip dan simpan ke dalam suatu berkas (file)
- `tar czf jalan/menuju/target.tar.gz -C jalan/menuju/direktori .` : buat (create) suatu arsip dengan tambahan kompresi gzip dari suatu direktori mengunakan alamat berkas relatif
- `tar xvf jalan/menuju/sumber.tar.ext` : extrak suatu berkas (file) arsip (biasa atau terkompres) menuju direktori saat ini dengan menampilkan rincian operasi (mode verbose)
- `tar xf jalan/menuju/sumber.tar.ext -C jalan/menuju/direktori` : extrak suatu berkas (file) arsip (biasa atau terkompres) menuju direktori target yang ditentukan
- `tar caf jalan/menuju/target.tar.xz jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : buat (create) suatu arsip terkompres dan simpan di dalam suatu berkas (file), menggunakan metode kompresi yang ditentukan secara otomatis berdasarkan nama ekstensi berkas tujuan
- `tar tvf jalan/menuju/sumber.tar` : tampilkan isi suatu berkas (file) tar secara rinci (mode verbose)
- `tar xf jalan/menuju/sumber.tar --wildcards "*.html"` : extrak kumpulan berkas yang namanya memenuhi pola kriteria yang ditentukan dari suatu berkas (file) arsip

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
