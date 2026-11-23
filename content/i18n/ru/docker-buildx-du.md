# docker buildx du

Показывать использование диска для сборщика.

## Usage
- `docker buildx du` : показать использование диска
- `docker buildx du --filter "description~=golang"` : отфильтровать вывод по определённому условию
- `docker buildx du --verbose` : показать подробный вывод
- `docker buildx du --format "table \{\{.ID\}\}    \{\{.Description\}\}"` : отформатировать вывод с использованием шаблона Go
- `docker buildx du --format json | jq .` : вывести в удобочитаемом формате JSON с использованием команды `jq`
- `docker buildx du --builder имя_сборщика` : показать использование диска для определённого сборщика

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/du/
