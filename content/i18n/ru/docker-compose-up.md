# docker compose up

Запускать Docker-сервисы, определённые в Compose-файле.

## Usage
- `docker compose up` : запустить все сервисы, определённые в Compose-файле
- `docker compose up -d` : запустить сервисы в фоновом режиме (detached mode)
- `docker compose up --build` : запустить сервисы и пересобрать образы перед запуском
- `docker compose up сервис1 сервис2 ...` : запустить только указанные сервисы
- `docker compose -f путь/к/файлу up` : запустить сервисы с указанием альтернативного Compose-файла
- `docker compose up --remove-orphans` : запустить сервисы и удалить осиротевшие контейнеры
- `docker compose up --scale сервис=количество` : запустить сервисы с масштабированием экземпляров
- `docker compose up --timestamps` : запустить сервисы и показать логи с временными метками

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/up/
