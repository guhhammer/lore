# docker container logs

Konteyner kaydını yazdırır.

## Usage
- `docker logs konteyner_ismi` : bir konteyner içindeki kayıtları yazdır
- `docker logs konteyner_ismi -f` : kayıtları yazdır ve izle
- `docker logs konteyner_ismi -n 5` : son 5 kaydı yazdır
- `docker logs konteyner_ismi -t` : kayıtları yazdır ve zaman damgaları ile iliştir
- `docker logs konteyner_ismi --until zaman` : belli bir konteyner çalışma zamanındaki (i.e. 23m, 10s, 2013-01-02T13:23:37) kayıtları yazdır

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
