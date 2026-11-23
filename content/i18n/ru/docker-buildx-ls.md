# docker buildx ls

Выводить список экземпляров сборщиков и связанных узлов.

## Usage
- `docker buildx ls` : вывести список экземпляров сборщиков
- `docker buildx ls --format "\{\{.NAME\}\}: \{\{.DriverEndpoint\}\}"` : отформатировать вывод с использованием Go-шаблона

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/ls/
