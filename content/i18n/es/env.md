# env

Muestra el entorno o ejecuta un programa en un entorno modificado.

## Usage
- `env` : muestra el entorno
- `env program` : ejecuta un programa. A menudo se utiliza en scripts después del shebang (#!) para buscar la ruta al programa
- `env -i program` : limpia el entorno y ejecuta un programa
- `env -u variable program` : elimina variable del entorno y ejecuta un programa
- `env variable=valor program` : establece una variable y ejecuta un programa
- `env variable1=value variable2=value variable3=value ... program` : establece una o más variables y ejecuta un programa
- `env -a custom_name program` : ejecuta un programa con un nombre diferente

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/env-invocation.html
