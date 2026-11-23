# bzip2

Suatu program pengompres berkas berbasis pengurutan blok.

## Usage
- `bzip2 jalan/menuju/berkas_untuk_dikompres` : kompres suatu berkas
- `bzip2 -d jalan/menuju/berkas_terkompres.bz2` : dekompres suatu berkas
- `bzip2 -dc jalan/menuju/berkas_terkompres.bz2` : dekompres suatu berkas menuju `stdout`
- `bzip2 -t jalan/menuju/berkas_terkompres.bz2` : ujikan keutuhan isi setiap berkas dalam suatu berkas arsip
- `bzip2 -v jalan/menuju/berkas_berkas_terkompres.bz2` : tampilkan nilai rasio kompresi untuk setiap berkas yang terproses dalam suatu arsip secara terperinci
- `bzip2 -f jalan/menuju/berkas_terkompres.bz2` : dekompres suatu berkas, dan gantikan isi berkas arsip tujuan secara paksa
- `bzip2 -h` : tampilkan bantuan

## Links
- More information: https://manned.org/bzip2
