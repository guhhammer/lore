# htmlq

Gunakan selektor CSS untuk mengekstrak konten dari berkas-berkas HTML.

## Usage
- `cat jalan/menuju/berkas.html | htmlq '.card'` : dapatkan seluruh elemen HTML yang memuat class `card`
- `cat jalan/menuju/berkas.html | htmlq -t 'p:first-of-type'` : dapatkan konten teks dari paragraf pertama
- `cat jalan/menuju/berkas.html | htmlq -a href 'a'` : tampilkan seluruh tautan dalam suatu laman
- `cat jalan/menuju/berkas.html | htmlq -r 'img' -r 'svg'` : hapus seluruh gambar dan elemen SVG dari suatu laman
- `htmlq -p -f jalan/menuju/input.html -o jalan/menuju/output.html` : simpan laman input secara cetak-cantik menuju suatu berkas output

## Links
- More information: https://github.com/mgdm/htmlq#usage
