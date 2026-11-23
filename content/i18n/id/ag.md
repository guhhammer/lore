# ag

The Silver Searcher. Seperti `ack`, namun bertujuan untuk lebih cepat daripadanya.

## Usage
- `ag foo` : cari file yang mengandung teks "foo", dan cetak baris teks yang cocok
- `ag foo jalan/menuju/direktori` : cari file yang mengandung teks "foo" dalam direktori tertentu
- `ag -l foo` : cari file yang mengandung teks "foo", namun hanya tampilkan daftar nama file saja
- `ag -i -o FOO` : cari file yang mengandung teks "FOO" tanpa memerhatikan perbedaan huruf besar-kecil (case-insensitive), dan hanya cetak teks yang cocok (jangan cetak kata-kata lainnya meskipun dalam baris yang sama)
- `ag foo -G bar` : cari file yang memiliki nama "bar" dan mengandung teks "foo"
- `ag '^ba(r|z)$'` : cari file yang memiliki teks yang memenuhi kriteria ekspresi reguler
- `ag -g foo` : cari file dengan nama yang memiliki teks "foo"

## Links
- More information: https://manned.org/ag
