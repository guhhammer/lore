# docker container logs

Cetak isi log sistem dalam kumpulan kontainer.

## Usage
- `docker logs nama_kontainer` : cetak isi log dalam suatu kontainer
- `docker logs nama_kontainer -f` : cetak isi log dan tetap ikuti untuk menerima pesan log baru
- `docker logs nama_kontainer -n 5` : hanya cetak 5 baris log terakhir
- `docker logs nama_kontainer -t` : cetak isi log beserta waktu di mana setiap pesan muncul
- `docker logs nama_kontainer --until waktu` : cetak isi log yang muncul sejak waktu lampau yang ditentukan (misalnya 23m, 10s, 2013-01-02T13:23:37)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
