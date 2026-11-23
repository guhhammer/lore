# docker buildx build

Compila una imagen a partir de un Dockerfile utilizando el motor BuildKit.

## Usage
- `docker buildx build .` : compila una imagen a partir del Dockerfile del directorio actual
- `docker buildx build -t image:tag .` : compila una imagen y la etiqueta
- `docker buildx build -f ruta/a/Dockerfile .` : compila una imagen utilizando un Dockerfile específico
- `docker buildx build --build-arg HTTP_PROXY=http://proxy.example.com --build-arg VERSION=1.0 .` : compila una imagen pasando variables de compilación
- `docker buildx build --no-cache .` : compila una imagen sin utilizar la caché de compilación
- `docker buildx build --load -t image:tag .` : compila una imagen y la carga en `docker images`
- `docker buildx build --platform linux/amd64,linux/arm64 --push -t registry.example.com/image:tag .` : compila para varias plataformas y la envía a un registro
- `docker buildx build --target nombre_etapa -t image:tag .` : compila una etapa específica a partir de un Dockerfile de varias etapas

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/build
