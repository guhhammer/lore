# docker container commit

Создавать новый образ из изменений контейнера.

## Usage
- `docker commit контейнер образ:тег` : создать образ из конкретного контейнера
- `docker commit -c "CMD команда" контейнер образ:тег` : применить инструкцию `CMD` Dockerfile к созданному образу
- `docker commit -c "ENV имя=значение" контейнер образ:тег` : применить инструкцию `ENV` Dockerfile к созданному образу
- `docker commit -a "автор" контейнер образ:тег` : создать образ с указанием автора в метаданных
- `docker commit -m "комментарий" контейнер образ:тег` : создать образ с указанием комментария в метаданных
- `docker commit -p false контейнер образ:тег` : создать образ без остановки контейнера во время коммита
- `docker commit --help` : показать справку

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/commit/
