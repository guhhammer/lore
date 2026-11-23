# docker image ls

Выводить список образов Docker.

## Usage
- `docker images` : вывести список всех образов Docker
- `docker images -a` : вывести список всех образов Docker, включая промежуточные
- `docker images -q` : вывести список в тихом режиме (только числовые идентификаторы)
- `docker images -f dangling=true` : вывести список всех образов Docker, не используемых ни одним контейнером
- `docker images "*имя*"` : вывести список образов, содержащих подстроку в имени
- `docker images --format "\{\{.ID\}\}\t\{\{.Size\}\}\t\{\{.Repository\}\}:\{\{.Tag\}\}" | sort -k 2 -h` : отсортировать образы по размеру

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/ls/
