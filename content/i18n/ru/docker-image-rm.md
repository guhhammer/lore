# docker image rm

Удалять образы Docker.

## Usage
- `docker rmi образ1 образ2 ...` : удалить один или несколько образов по имени
- `docker rmi -f образ` : принудительно удалить образ
- `docker rmi --no-prune образ` : удалить образ без удаления родительских образов без тегов
- `docker rmi --help` : показать справку

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/rm/
