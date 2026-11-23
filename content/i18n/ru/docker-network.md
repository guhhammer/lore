# docker network

Создавать сети Docker и управлять ими.

## Usage
- `docker network ls` : вывести список всех доступных и настроенных сетей в демоне Docker
- `docker network create -d имя_драйвера имя_сети` : создать пользовательскую сеть
- `docker network inspect имя_сети1 имя_сети2 ...` : показать подробную информацию об одной или нескольких сетях
- `docker network connect имя_сети имя_контейнера|идентификатор` : подключить контейнер к сети по имени или идентификатору
- `docker network disconnect имя_сети имя_контейнера|идентификатор` : отключить контейнер от сети
- `docker network prune` : удалить все неиспользуемые сети (не связанные ни с одним контейнером)
- `docker network rm имя_сети1 имя_сети2 ...` : удалить одну или несколько неиспользуемых сетей

## Links
- More information: https://docs.docker.com/reference/cli/docker/network/
