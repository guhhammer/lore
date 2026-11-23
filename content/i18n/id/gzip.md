# gzip

Kompres/dekompres berkas/fail komputer dengan kompresi `gzip` (LZ77).

## Usage
- `gzip jalan/menuju/berkas` : kompres sebuah berkas, menggantinya dengan arsip `gzip`
- `gzip -d jalan/menuju/berkas.gz` : dekompres sebuah berkas, menggantinya dengan versi asli yang tidak terkompres
- `gzip -v jalan/menuju/berkas` : tampilkan daftar nama dan persentase pengurangan beban untuk setiap berkas terkompres
- `gzip -k jalan/menuju/berkas` : kompres sebuah berkas dengan tetap mempertahankan berkas asli
- `gzip -c jalan/menuju/berkas >jalan/menuju/berkas_terkompres.gz` : kompres sebuah berkas dengan menentukan nama berkas output
- `gzip -cd jalan/menuju/berkas.gz >jalan/menuju/berkas_dekompres` : dekompres sebuah arsip `gzip` dengan menentukan nama berkas output
- `gzip -1..9 -c jalan/menuju/berkas >jalan/menuju/berkas_terkompres.gz` : tentukan level kompresi. 1 berarti yang tercepat (kompresi rendah), 9 berarti yang terlambat (kompresi tinggi), 6 berarti default
- `gzip -l jalan/menuju/berkas.txt.gz` : tampilkan daftar isi suatu berkas arsip

## Links
- More information: https://www.gnu.org/software/gzip/manual/gzip.html
