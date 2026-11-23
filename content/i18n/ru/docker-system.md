# docker system

Управлять данными Docker и отображать общесистемную информацию.

## Usage
- `docker system df` : показать использование диска Docker
- `docker system df -v` : показать подробную информацию об использовании диска
- `docker system prune` : удалить неиспользуемые данные (добавить `--volumes` для удаления неиспользуемых томов)
- `docker system prune --filter "until=часыhминутыm"` : удалить неиспользуемые данные, созданные более указанного времени назад
- `docker system prune -a --volumes` : удалить все неиспользуемые данные
- `docker system events` : показать события от демона Docker в реальном времени
- `docker system events -f 'type=container' --format '\{\{json .\}\}'` : показать события от контейнеров в реальном времени в формате JSON Lines
- `docker system info` : показать общесистемную информацию

## Links
- More information: https://docs.docker.com/reference/cli/docker/system/
