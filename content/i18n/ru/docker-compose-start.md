# docker compose start

Запускать существующие контейнеры для сервисов.

## Usage
- `docker compose start` : запустить существующие контейнеры для всех сервисов
- `docker compose start сервис1 сервис2 ...` : запустить существующие контейнеры для одного или нескольких сервисов
- `docker compose start --dry-run` : симулировать запуск существующих контейнеров
- `docker compose start --wait` : запустить существующие контейнеры и ждать, пока сервисы будут запущены или здоровы
- `docker compose start --wait --wait-timeout секунды` : запустить существующие контейнеры и ждать указанное количество секунд

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/start/
