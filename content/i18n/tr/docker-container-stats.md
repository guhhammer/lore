# docker container stats

Konteynerler için kaynak kullanım istatistiklerinin canlı yayınını görüntüle.

## Usage
- `docker stats` : çalışan tüm konteynerlerin aynak kullanım istatistiklerinin canlı yayınını görüntüle
- `docker stats container_ismi` : boşluk ile ayrılmış bir listedeki konteynerlerin canlı yayınını görüntüle
- `docker stats --format "\{\{.Name\}\}:\t\{\{.CPUPerc\}\}"` : konteyner'in CPU kullanım yüzdesini göstermek için sütun formatını değiştir
- `docker stats -a` : tüm (çalışan veya durmuş) konteynerler için istatistikleri görüntüle
- `docker stats --no-stream` : i̇statistikleri canlı yayınlamayı durdur ve yalnızca mevcut durumdaki istatistikleri görüntüle

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stats/
