# docker volume

Docker birimlerini (volume) yönet.

## Usage
- `docker volume create birim_ismi` : bir birim oluştur
- `docker volume create --label etiket birim_ismi` : belirli bir etiketle birim oluştur
- `docker volume create -o type=tmpfs -o device=tmpfs -o o=size=100m,uid=1000 birim_ismi` : boyutu 100 MiB ve uid değeri 1000 olan bir `tmpfs` birimi oluştur
- `docker volume ls` : tüm birimleri listele
- `docker volume rm birim_ismi` : bir birimi sil
- `docker volume inspect birim_ismi` : bir birim hakkında bilgi görüntüle
- `docker volume prune` : kullanılmayan tüm yerel birimleri sil
- `docker volume alt_komut --help` : bir alt komut için yardım görüntüle

## Links
- More information: https://docs.docker.com/reference/cli/docker/volume/
