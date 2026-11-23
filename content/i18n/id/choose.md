# choose

Alternatif ramah-manusia dan cepat terhadap perintah cut dan (terkadang) awk.

## Usage
- `choose 4` : cetak item ke-5 dari suatu baris (dimulai dari 0)
- `choose -f ':' 0 2 4` : cetak item pertama, ke-3, dan ke-5 dari suatu baris, di mana kumpulan item dipisahkan menggunakan karakter ':' daripada spasi
- `choose 1:4` : cetak isi seluruh item dari urutan ke-2 menuju ke-5 dalam baris, termasuk item ke-5
- `choose -x 1:4` : cetak isi seluruh item dari urutan ke-2 menuju ke-5 dalam baris, kecuali item ke-5
- `choose :2` : cetak isi seluruh item dari awal baris menuju item ke-3
- `choose -x :2` : cetak isi seluruh item dari awal baris menuju item ke-3 (eksklusif)
- `choose 2:` : cetak isi seluruh item dari urutan ke-3 hingga akhir baris
- `choose -1` : cetak item terakhir dari suatu baris

## Links
- More information: https://github.com/theryangeary/choose#usage
