# octo

Herramientas de línea de comandos para Octopus Deploy.

## Usage
- `octo pack --id paquete` : crea un paquete
- `octo push --package paquete` : sube un paquete a un repositorio en el servidor de Octopus
- `octo create-release --project project_name --packageversion version` : crea una versión
- `octo deploy-release --project project_name --packageversion version --deployto environment_name --tenant deployment_target` : despliega una versión

## Links
- More information: https://octopus.com/docs/cli/octopus-cli
