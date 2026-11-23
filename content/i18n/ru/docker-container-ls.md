# docker container ls

Выводить список контейнеров Docker.

## Usage
- `docker ps` : вывести список запущенных контейнеров Docker
- `docker ps -a` : вывести список всех контейнеров Docker (запущенных и остановленных)
- `docker ps -l` : показать последний созданный контейнер (включая все состояния)
- `docker ps -f "name=имя"` : отфильтровать контейнеры, содержащие подстроку в имени
- `docker ps -f "ancestor=образ:тег"` : отфильтровать контейнеры, использующие указанный образ в качестве предка (ancestor)
- `docker ps -f "exited=код" -a` : отфильтровать контейнеры по коду завершения
- `docker ps -f "status=статус"` : отфильтровать контейнеры по статусу (created, running, removing, paused, exited и dead)
- `docker ps -f "volume=путь/к/каталогу" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : отфильтровать контейнеры, которые монтируют определённый том или имеют том, смонтированный по определённому пути

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
