# docker volume

Управлять томами Docker.

## Usage
- `docker volume create имя_тома` : создать том
- `docker volume create --label метка имя_тома` : создать том с определённой меткой
- `docker volume create -o type=tmpfs -o device=tmpfs -o o=size=100m,uid=1000 имя_тома` : создать том `tmpfs` размером 100 МиБ и uid 1000
- `docker volume ls` : вывести список всех томов
- `docker volume rm имя_тома` : удалить том
- `docker volume inspect имя_тома` : показать информацию о томе
- `docker volume prune` : удалить все неиспользуемые локальные тома
- `docker volume подкоманда --help` : показать справку по подкоманде

## Links
- More information: https://docs.docker.com/reference/cli/docker/volume/
