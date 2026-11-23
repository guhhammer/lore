# docker

Atur kontainer Docker dan image. Beberapa subperintah seperti `run` mempunyai dokumentasi terpisah.

## Usage
- `docker ps -a` : tampilkan semua daftar kontainer Docker (yang sedang berjalan dan berhenti)
- `docker run --name nama_kontainer citra` : nyalakan sebuah kontainer dari citra (image), dengan nama kustom
- `docker container start|stop nama_kontainer` : nyalakan atau menghentikan kontainer yang tersedia
- `docker pull citra` : tarik citra dari registri Docker
- `docker images` : tampilkan daftar citra Docker yang telah diunduh
- `docker exec -it nama_kontainer sh` : buka sesi shell didalam sebuah kontainer yang sedang berjalan
- `docker rm nama_kontainer` : hapus kontainer yang sedang berhenti
- `docker logs -f nama_kontainer` : ambil dan ikuti semua log dari sebuah kontainer

## Links
- More information: https://docs.docker.com/reference/cli/docker/
