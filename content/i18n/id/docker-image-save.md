# docker image save

Ekspor kumpulan citra Docker menuju berkas arsip.

## Usage
- `docker save citra:tag >jalan/menuju/berkas.tar` : simpan suatu citra dengan menangkap isi dari `stdout` menuju suatu berkas arsip `.tar`
- `docker save -o jalan/menuju/berkas.tar image:tag` : simpan suatu citra menuju suatu arsip `.tar`
- `docker save -o jalan/menuju/berkas.tar nama_citra` : simpan seluruh tag dalam proses ekspor citra
- `docker save -o jalan/menuju/berkas.tar nama_citra:tag1 nama_citra:tag2 ...` : tentukan daftar tag yang hendak disimpan dalam proses ekspor citra

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
