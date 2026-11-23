# docker buildx create

Создавать новый экземпляр сборщика.

## Usage
- `docker buildx create` : создать новый экземпляр сборщика с использованием контекста Docker по умолчанию
- `docker buildx create --name имя_сборщика` : создать новый экземпляр сборщика с определённым именем
- `docker buildx create --name имя_сборщика --use` : создать новый экземпляр сборщика и сразу установить его как текущий активный
- `docker buildx create --driver docker-container|kubernetes|remote|...` : создать новый экземпляр сборщика с использованием определённого драйвера (по умолчанию `docker`)
- `docker buildx create --platform linux/amd64,linux/arm64,...` : создать новый экземпляр сборщика с определёнными поддерживаемыми платформами
- `docker buildx create --name имя_сборщика --append контекст|точка_подключения` : добавить новый узел к существующему сборщику
- `docker buildx create --buildkitd-flags "--debug --debugaddr 0.0.0.0:6666"` : создать новый экземпляр сборщика с определёнными флагами демона BuildKit
- `docker buildx create --name имя_сборщика --bootstrap` : создать новый экземпляр сборщика и сразу запустить его

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/create/
