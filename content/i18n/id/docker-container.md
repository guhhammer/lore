# docker container

Atur kumpulan kontainer Docker.

## Usage
- `docker ps` : tampilkan daftar kontainer Docker yang sedang berjalan
- `docker start nama_kontainer1 nama_kontainer2 ...` : jalankan satu atau beberapa kontainer yang dihentikan
- `docker kill nama_kontainer1 nama_kontainer2 ...` : matikan satu atau beberapa kontainer
- `docker stop nama_kontainer1 nama_kontainer2 ...` : hentikan satu atau beberapa kontainer
- `docker pause nama_kontainer1 nama_kontainer2 ...` : jedakan seluruh proses dalam satu atau beberapa kontainer
- `docker container inspect nama_kontainer1 nama_kontainer2 ...` : tampilkan rincian informasi mengenai satu atau beberapa kontainer
- `docker export nama_kontainer` : arsip suatu sistem penyimpanan pada kontainer ke dalam berkas `.tar`
- `docker commit nama_kontainer` : buat suatu citra (image) baru atas perubahan-perubahan yang dilakukan di dalam suatu kontainer

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/
