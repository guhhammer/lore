# docker compose

Запускать и управлять многоконтейнерными Docker-приложениями. Некоторые подкоманды, такие как `up`, `down`, `logs`, `stop`, имеют собственную документацию по использованию.

## Usage
- `docker compose ps` : вывести список всех запущенных контейнеров
- `docker compose up -d` : создать и запустить все контейнеры в фоновом режиме, используя файл `docker-compose.yml` из текущего каталога
- `docker compose up --build` : запустить все контейнеры, пересобрать при необходимости
- `docker compose -p имя_проекта -f путь/к/файлу up` : запустить все контейнеры, указав имя проекта и альтернативный Compose-файл
- `docker compose stop` : остановить все запущенные контейнеры
- `docker compose down --rmi all -v` : остановить и удалить все контейнеры, сети, образы и тома
- `docker compose logs -f` : просмотреть логи всех контейнеров
- `docker compose logs -f имя_контейнера` : просмотреть логи конкретного контейнера

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
