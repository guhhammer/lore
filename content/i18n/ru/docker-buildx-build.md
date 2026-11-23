# docker buildx build

Собирать образ из Dockerfile с использованием движка BuildKit.

## Usage
- `docker buildx build .` : собрать образ из Dockerfile в текущем каталоге
- `docker buildx build -t образ:тег .` : собрать образ и присвоить ему тег
- `docker buildx build -f путь/к/Dockerfile .` : собрать образ с использованием определённого Dockerfile
- `docker buildx build --build-arg HTTP_PROXY=http://proxy.example.com --build-arg VERSION=1.0 .` : собрать образ с передачей переменных времени сборки
- `docker buildx build --no-cache .` : собрать образ без использования кэша сборки
- `docker buildx build --load -t образ:тег .` : собрать образ и загрузить его в `docker images`
- `docker buildx build --platform linux/amd64,linux/arm64 --push -t registry.example.com/образ:тег .` : собрать для нескольких платформ и отправить в реестр
- `docker buildx build --target имя_этапа -t образ:тег .` : собрать определённый этап из многоэтапного Dockerfile

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/build
