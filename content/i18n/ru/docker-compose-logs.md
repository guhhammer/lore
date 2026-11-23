# docker compose logs

Просматривать вывод контейнеров в Docker Compose приложении.

## Usage
- `docker compose logs` : просмотреть логи всех сервисов
- `docker compose logs имя_сервиса` : просмотреть логи конкретного сервиса
- `docker compose logs -f` : просмотреть логи и отслеживать новый вывод (как `tail --follow`)
- `docker compose logs -t` : просмотреть логи с временными метками
- `docker compose logs -n n` : просмотреть только последние `n` строк логов для каждого контейнера
- `docker compose logs --since временная_метка` : просмотреть логи начиная с определённого времени
- `docker compose logs --until временная_метка` : просмотреть логи до определённого времени
- `docker compose logs сервис1 сервис2 ...` : просмотреть логи нескольких конкретных сервисов

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/logs/
