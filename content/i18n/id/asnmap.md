# asnmap

Suatu alat baris perintah berbasis Go untuk memetakan rentang jaringan organisasi beserta kepemilikannya menggunakan data ASN. Catatan: Membutuhkan sebuah kunci API dari layanan ProjectDiscovery Cloud Platform.

## Usage
- `asnmap -a AS5650 -silent` : cari rentang CIDR untuk suatu ASN
- `asnmap -i 100.19.12.21 -silent` : cari rentang CIDR untuk suatu alamat IP
- `asnmap -d example.com -silent` : cari rentang CIDR untuk suatu nama domain
- `asnmap -org GOOGLE -silent` : cari rentang CIDR untuk suatu nama organisasi
- `asnmap -f jalan/menuju/target.txt -silent` : cari rentang CIDR berdasarkan daftar target pencarian dalam suatu berkas
- `asnmap -d facebook.com -j -silent` : keluarkan hasil dalam format JSON
- `asnmap -a AS394161 -c -silent` : keluarkan hasil dalam format CSV
- `asnmap -up` : perbarui asnmap menuju versi terkini

## Links
- More information: https://github.com/projectdiscovery/asnmap#usage
