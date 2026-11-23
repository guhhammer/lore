# exif

Lihat dan ubah informasi metadata EXIF pada berkas-berkas JPEG.

## Usage
- `exif jalan/menuju/gambar.jpg` : tampilkan daftar informasi EXIF yang terdapat pada suatu berkas gambar
- `exif -l --no-fixup gambar.jpg` : tampilkan daftar jenis tag informasi EXIF dalam format tabel, termasuk apakah tag tersebut terdapat dalam suatu gambar
- `exif -e -o thumbnail.jpg gambar.jpg` : ekstrak gambar pratinjau (thumbnail) dari suatu gambar menuju `thumbnail.jpg`
- `exif --ifd 0 -t "Model" -m gambar.jpg` : tampilkan isi mentahan terhadap tag metadata "Model" dalam suatu gambar
- `exif -o new.jpg --ifd 0 -t "Artist" --set-value "John Smith" --no-fixup gambar.jpg` : ganti nilai tag metadata "Artist" menjadi John Smith, dan simpan perubahan menuju berkas baru di `new.jpg`

## Links
- More information: https://manned.org/exif
