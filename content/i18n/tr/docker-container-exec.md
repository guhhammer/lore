# docker container exec

Halihazırda çalışan bir Docker konteyneri üstünde komut çalıştır.

## Usage
- `docker exec -it konteyner_ismi /bin/bash` : halihazırda çalışan bir konteynerin üstünde interaktif bir kabuk oturumunu çalıştır
- `docker exec -d konteyner_ismi komut` : halihazırda çalışan bir konteynerin üstüne arkaplanda çalışmak üzere (ayrılmış) bir komut çalıştır
- `docker exec -it -w örnek/dizin konteyner_ismi komut` : belirtilen bir komutu üstünde çalıştırmak adına çalışan dizini seç
- `docker exec -i -d konteyner_ismi komut` : varolan konteyner üstünde arkaplanda çalışmak üzere bir komut çalıştır ancak `stdin`'i açık tut
- `docker exec -it -e değişken_ismi=value konteyner_ismi /bin/bash` : çalışmakta olan bir Bash oturumu içinde bir çevre değişkeni belirle
- `docker exec -u kullanıcı konteyner_ismi komut` : belirtilmiş bir kullanıcı olarak komut çalıştır

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
