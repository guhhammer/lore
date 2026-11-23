# docker secret

Управлять секретами Docker swarm.

## Usage
- `команда | docker secret create имя_секрета -` : создать новый секрет из `stdin`
- `docker secret create имя_секрета путь/к/файлу` : создать новый секрет из файла
- `docker secret ls` : вывести список всех секретов
- `docker secret inspect --pretty имя_секрета1 имя_секрета2 ...` : показать подробную информацию об одном или нескольких секретах в удобочитаемом формате
- `docker secret rm имя_секрета1 имя_секрета2 ...` : удалить один или несколько секретов

## Links
- More information: https://docs.docker.com/reference/cli/docker/secret/
