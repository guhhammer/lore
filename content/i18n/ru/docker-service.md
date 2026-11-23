# docker service

Управлять сервисами на демоне Docker.

## Usage
- `docker service ls` : вывести список сервисов на демоне Docker
- `docker service create --name имя_сервиса образ:тег` : создать новый сервис
- `docker service inspect имя_или_id_сервиса1 имя_или_id_сервиса2 ...` : показать подробную информацию об одном или нескольких сервисах
- `docker service ps имя_или_id_сервиса1 имя_или_id_сервиса2 ...` : вывести список задач одного или нескольких сервисов
- `docker service scale имя_сервиса=количество_реплик` : масштабировать до определённого количества реплик для списка сервисов через пробел
- `docker service rm имя_или_id_сервиса1 имя_или_id_сервиса2 ...` : удалить один или несколько сервисов

## Links
- More information: https://docs.docker.com/reference/cli/docker/service/
