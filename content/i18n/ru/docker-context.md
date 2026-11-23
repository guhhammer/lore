# docker context

Переключаться между контекстами для управления несколькими окружениями Docker.

## Usage
- `docker context create имя_контекста --docker "host=tcp://удалённый-хост:2375"` : создать контекст с использованием определённой конечной точки Docker
- `docker context create имя_контекста` : создать контекст на основе переменной окружения `$DOCKER_HOST`
- `docker context use имя_контекста` : переключиться на контекст
- `docker context ls` : вывести список всех контекстов

## Links
- More information: https://docs.docker.com/reference/cli/docker/context/
