# docker node

Управлять узлами Docker Swarm.

## Usage
- `docker node ls` : вывести список узлов в swarm
- `docker node ps узел1 узел2 узел3 ...` : вывести список задач, выполняющихся на одном или нескольких узлах (по умолчанию текущий узел)
- `docker node inspect узел1 узел2 узел3 ...` : показать подробную информацию об одном или нескольких узлах
- `docker node promote узел1 узел2 узел3 ...` : повысить один или несколько узлов до менеджера в swarm
- `docker node demote узел1 узел2 узел3 ...` : понизить один или несколько узлов с менеджера в swarm
- `docker node rm узел1 узел2 узел3 ...` : удалить один или несколько узлов из swarm
- `docker node update --availability|role|label-add|... active|worker|... узел1` : обновить метаданные узла, такие как доступность, метки или роли

## Links
- More information: https://docs.docker.com/reference/cli/docker/node/
