# docker image save

Экспортировать образы Docker в архив.

## Usage
- `docker save образ:тег >путь/к/файлу.tar` : сохранить образ, перенаправив `stdout` в архив `.tar`
- `docker save -o путь/к/файлу.tar образ:тег` : сохранить образ в архив `.tar`
- `docker save -o путь/к/файлу.tar имя_образа` : сохранить все теги образа
- `docker save -o путь/к/файлу.tar имя_образа:тег1 имя_образа:тег2 ...` : выборочно сохранить определённые теги образа

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
