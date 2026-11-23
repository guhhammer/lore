# docker compose down

Останавливать и удалять контейнеры, сети, образы и тома, созданные командой `docker compose up`.

## Usage
- `docker compose down` : остановить и удалить все контейнеры и сети
- `docker compose down --rmi all` : остановить и удалить контейнеры, сети и все образы, используемые сервисами
- `docker compose down --rmi local` : остановить и удалить контейнеры, сети и только образы без пользовательского тега
- `docker compose down -v` : остановить и удалить контейнеры, сети и все тома
- `docker compose down --remove-orphans` : остановить и удалить всё, включая осиротевшие контейнеры
- `docker compose -f путь/к/файлу down` : остановить и удалить контейнеры с использованием альтернативного Compose-файла
- `docker compose down -t таймаут` : остановить и удалить контейнеры с указанием таймаута в секундах
- `docker compose down --remove-orphans -v` : удалить контейнеры для сервисов, не определённых в Compose-файле

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/down/
