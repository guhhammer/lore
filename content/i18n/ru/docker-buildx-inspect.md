# docker buildx inspect

Показывать информацию о текущем или указанном экземпляре сборщика.

## Usage
- `docker buildx inspect` : показать информацию о текущем экземпляре сборщика
- `docker buildx inspect имя_сборщика` : показать информацию об указанном экземпляре сборщика по имени
- `docker buildx inspect --bootstrap` : убедиться, что сборщик запущен перед проверкой
- `docker buildx inspect --timeout секунды` : переопределить таймаут загрузки статуса сборщика (по умолчанию 20 секунд)

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/inspect
