# ack

Sebuah alat pencari teks seperti `grep` yang dikhususkan bagi para pengembang perangkat lunak.

## Usage
- `ack "pola_pencarian"` : cari file dalam direktori saat ini yang mengandung string atau kriteria dalam ekspresi reguler
- `ack -i "pola_pencarian"` : cari tanpa mementingkan perbedaan huruf besar-kecil dalam kriteria (case-insensitive)
- `ack -o "pola_pencarian"` : cari untuk baris-baris dalam file yang memenuhi kriteria, namun hanya cetak teks yang memenuhinya saja (jangan cetak kata-kata lainnya meskipun dalam baris yang sama)
- `ack -t ruby "pola_pencarian"` : hanya cari file dengan tipe tertentu (seperti `ruby` untuk mencari file `.rb`,`.erb`, `.rake`, `Rakefile` dan sebagainya)
- `ack -t noruby "pola_pencarian"` : jangan cari file dengan tipe tertentu
- `ack -c -h "pola_pencarian"` : hitung total teks/string yang ditemukan
- `ack -c -l "pola_pencarian"` : hanya cetak nama file dan total penemuan dalam file tersebut saja
- `ack --help-types` : tampilkan daftar kombinasi nilai yang dapat dipakai dalam atribut `--type`

## Links
- More information: https://beyondgrep.com/documentation/
