# docker

Docker konteyner ve imgelerini yönetir. `run` gibi bazı alt komutların kendi dökümantasyonu bulunmaktadır.

## Usage
- `docker ps -a` : tüm (çalışan veya duran) Docker konteynerlerini listele
- `docker run --name konteyner_ismi imge` : bir imgeden özel bir isimle konteyner başlat
- `docker start|stop konteyner_ismi` : varolan bir konteyneri başlat veya durdur
- `docker pull imge` : bir Docker kaydından imge çek
- `docker exec -it konteyner_ismi sh` : halihazırda çalışan bir konteyner içinde komut istemcisi aç
- `docker rm konteyner_ismi` : durmuş bir konteyneri sil
- `docker logs -f konteyner_ismi` : bir konteynerin kaydını çek ve takip et

## Links
- More information: https://docs.docker.com/reference/cli/docker/
