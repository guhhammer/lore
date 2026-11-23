# rsync

Transfer kumpulan berkas baik menuju atau dari suatu host jarak jauh (namun tidak antara dua host jarak jauh), secara konfigurasi bawaan menggunakan SSH. Untuk mendefinisikan suatu alamat sumber jarak jauh, gunakan `user@host:jalan/menuju/berkas_atau_direktori`.

## Usage
- `rsync jalan/menuju/sumber jalan/menuju/tujuan` : transfer suatu berkas
- `rsync -a jalan/menuju/sumber jalan/menuju/tujuan` : gunakan mode arsip (salin direktori secara rekursif, salin tautan simbolik tanpa menyelesaikan, dan pertahankan izin, kepemilikan, dan waktu modifikasi)
- `rsync -zvhP jalan/menuju/sumber jalan/menuju/tujuan` : kompres data saat dikirim ke tujuan, tampilkan informasi kemajuan secara verbose dan dapat dibaca manusia, dan simpan sebagian file yang ditransfer jika terganggu
- `rsync -r jalan/menuju/sumber jalan/menuju/tujuan` : salin kumpulan direktori secara rekursif
- `rsync -r jalan/menuju/sumber/ jalan/menuju/tujuan` : transfer isi direktori, tetapi bukan direktori itu sendiri
- `rsync -auL jalan/menuju/sumber jalan/menuju/tujuan` : gunakan mode arsip, selesaikan tautan simbolik, dan lewati file yang lebih baru di tujuan
- `rsync -r --delete rsync://host:jalan/menuju/sumber jalan/menuju/tujuan` : transfer direktori dari host jarak jauh yang menjalankan `rsyncd` dan hapus file di tujuan yang tidak ada di sumber
- `rsync -e 'ssh -p port' --info=progress2 host:jalan/menuju/sumber jalan/menuju/tujuan` : transfer file melalui SSH menggunakan port yang berbeda dari default (22) dan tampilkan kemajuan proses secara global

## Links
- More information: https://download.samba.org/pub/rsync/rsync.1
