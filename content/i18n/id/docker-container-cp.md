# docker container cp

Salin kumpulan berkas dan direktori antara sistem manajemen berkas perangkat komputer induk (host) dan dalam kontainer.

## Usage
- `docker cp jalan/menuju/berkas_atau_direktori_perangkat_induk nama_kontainer:jalan/menuju/berkas_atau_direktori_dalam_kontainer` : salin suatu berkas atau direktori dari penyimpanan perangkat induk menuju suatu kontainer
- `docker cp nama_kontainer:jalan/menuju/berkas_atau_direktori_dalam_kontainer jalan/menuju/berkas_atau_direktori_perangkat_induk` : salin suatu berkas atau direktori dari suatu kontainer menuju penyimpanan perangkat induk
- `docker cp -L jalan/menuju/symlink_perangkat_induk nama_kontainer:jalan/menuju/berkas_atau_direktori_dalam_kontainer` : salin suatu berkas atau direktori dari penyimpanan perangkat induk menuju suatu kontainer, dengan mengikuti tautan symbolik / symlink (salin berkas-berkas yang tertaut secara langsung, bukan definisi tautan symlink tersebut)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
