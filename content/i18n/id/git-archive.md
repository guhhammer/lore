# git archive

Buat sebuah arsip direktori berdasarkan cabang/tree tertentu.

## Usage
- `git archive -v HEAD` : buat sebuah arsip `.tar` berisikan isi dari tree HEAD saat ini, kemudian tampilkan isi file arsip mentah menuju `stdout`
- `git archive -v --format zip HEAD` : buat sebuah arsip zip dari tree HEAD saat ini, kemudian tampilkan isi file arsip mentah menuju `stdout`
- `git archive -v -o jalan/menuju/berkas.zip HEAD` : lakukan hal yang sama, namun simpan arsip zip ke dalam suatu direktori
- `git archive -o jalan/menuju/berkas.tar nama_cabang` : buat arsip `.tar` dari komit terakhir pada cabang tertentu
- `git archive -o jalan/menuju/berkas.tar HEAD:jalan/menuju/direktori` : buat arsip `.tar` berdasaran subdirektori tertentu pada suatu repositori Git
- `git archive -o jalan/menuju/berkas.tar --prefix jalan/untuk/dibubuhkan/ HEAD` : bubuhkan nama jalur pada awal nama setiap file, untuk diarsipkan di dalam direktori tertentu

## Links
- More information: https://git-scm.com/docs/git-archive
