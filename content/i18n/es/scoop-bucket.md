# scoop bucket

Administra buckets: repositorios Git que contienen archivos que describen cómo scoop instala aplicaciones. Si Scoop no sabe dónde se encuentra el bucket, debe especificarse la ubicación del repositorio.

## Usage
- `scoop bucket list` : lista todos los buckets actualmente en uso
- `scoop bucket known` : lista todos los buckets conocidos
- `scoop bucket add nombre` : agrega un bucket conocido por su nombre
- `scoop bucket add nombre https://example.com/repository.git` : agrega un bucket desconocido por su nombre y URL del repositorio Git
- `scoop bucket rm nombre` : elimina un bucket por su nombre

## Links
- More information: https://github.com/ScoopInstaller/Scoop/wiki/Buckets
