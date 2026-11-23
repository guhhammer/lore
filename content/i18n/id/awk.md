# awk

Bahasa pemrograman serbaguna yang seringkali digunakan untuk memproses berkas. Note: Implementasi AWK yang berbeda (misalnya, gawk, nawk, atau lainnya) akan membuat tautan simbolik dari nama file tertentu ke file biner utama mereka.

## Usage
- `awk '{print $5}' jalan/menuju/berkas` : cetak kolom (atau bidang) kelima dalam berkas yang dipisahkan spasi
- `awk '/foo/ {print $2}' jalan/menuju/berkas` : cetak kolom kedua dari baris yang mengandung "foo" dalam berkas yang dipisahkan spasi
- `awk -F ',' '{print $NF}' jalan/menuju/berkas` : cetak kolom terakhir dari setiap baris dalam sebuah berkas, menggunakan koma (bukan spasi) sebagai pemisah bidang
- `awk '{s+=$1} END {print s}' jalan/menuju/berkas` : jumlahkan nilai di kolom pertama dari sebuah berkas dan cetak totalnya
- `awk 'NR%3==1' jalan/menuju/berkas` : cetak setiap baris ketiga dimulai dari baris pertama
- `awk '{if ($1 == "foo") print "Cocok persis foo"; else if ($1 ~ "bar") print "Cocok sebagian bar"; else print "Baz"}' lokasi/berkas` : cetak nilai yang berbeda berdasarkan kondisi
- `awk '($10 >= nilai_min && $10 <= nilai_maks)' jalan/menuju/berkas` : cetak semua baris yang nilai kolom ke-10nya berada di antara nilai minimum dan maksimum
- `awk 'BEGIN {FS=":";printf "%-20s %6s %25s\n", "Nama", "UID", "Shell"} $4 >= 1000 {printf "%-20s %6d %25s\n", $1, $4, $7}' /etc/passwd` : cetak tabel pengguna dengan UID >= 1000 beserta header dan output yang terformat, menggunakan titik dua sebagai pemisah (`%-20s` berarti: string 20 karakter rata kiri, `%6s` berarti: string 6 karakter rata kanan)

## Links
- More information: https://github.com/onetrueawk/awk
