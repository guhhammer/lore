# calibredb

Kelola suatu pangkalan data perpustakaan buku digital. Bagian dari aplikasi perpustakaan buku digital Calibre.

## Usage
- `calibredb list` : tampilkan daftar judul dan informasi tambahan terkait buku-buku digital yang telah terdaftar dalam perpustakaan
- `calibredb list --search kata_kunci` : cari kumpulan buku dengan informasi tambahan
- `calibredb search kata_kunci` : hanya tampilkan nomor induk (id) dari hasil pustaka pencarian
- `calibredb add jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : masukkan satu atau beberapa buku baru ke dalam perpustakaan
- `calibredb add -r jalan/menuju/direktori` : masukkan seluruh buku dalam suatu direktori secara rekursif
- `calibredb remove id1 id2 ...` : hapus satu atau beberapa buku dari perpustakaan. Anda perlu memasukkan nomor-nomor induk (lihat keterangan di atas)

## Links
- More information: https://manual.calibre-ebook.com/generated/en/calibredb.html
