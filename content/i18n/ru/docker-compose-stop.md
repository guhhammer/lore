# docker compose stop

Останавливать запущенные контейнеры без их удаления.

## Usage
- `docker compose stop` : остановить все запущенные сервисы
- `docker compose stop сервис_1 сервис_2 ...` : остановить определённые сервисы
- `docker compose stop -t секунды` : остановить с пользовательским таймаутом завершения в секундах
- `docker compose -f путь/к/compose_файлу stop` : остановить сервисы, определённые в конкретном compose-файле
- `docker compose stop --dry-run` : симулировать выполнение операций без их фактического выполнения
- `docker compose stop -t секунды сервис_1 сервис_2 ...` : остановить определённые сервисы с пользовательским таймаутом

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/stop
