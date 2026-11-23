# git blame

Tampilkan informasi kode hash dan pelaku komit terakhir pada setiap baris dalam suatu berkas teks.

## Usage
- `git blame jalan/menuju/berkas` : tampilkan berkas teks beserta informasi nama pelaku dan kode hash komit terakhir pada akhir setiap baris teks
- `git blame -e jalan/menuju/berkas` : tampilkan berkas dengan informasi komit menggunakan alamat surel/email daripada nama pelaku
- `git blame komit jalan/menuju/berkas` : tampilkan informasi nama pelaku dan kode hash komit terakhir pada berkas yang disimpan dalam komit tertentu
- `git blame komit~ jalan/menuju/berkas` : tampilkan informasi nama pelaku dan kode hash komit terakhir pada berkas yang disimpan sebelum komit tertentu
- `git blame -L 123 jalan/menuju/berkas` : tampilkan suatu berkas beserta informasi pelaku komit pada nomor baris teks yang ditentukan
- `git blame -L nomor_baris_awal,nomor_baris_akhir jalan/menuju/berkas` : lakukan anotasi informasi pelaku komit dalam rentang nomor baris teks yang ditentukan
- `git blame -L '/isi_teks/',+10 jalan/menuju/berkas` : lakukan anotasi dalam 10 baris pertama dalam berkas dengan baris pertama memiliki isi teks yang ditentukan
- `git blame -w -C -C -C jalan/menuju/berkas` : lakukan anotasi dalam suatu berkas tanpa menghiraukan karakter spasi dan pergerakan baris teks

## Links
- More information: https://git-scm.com/docs/git-blame
