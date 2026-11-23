# az webapp

Administra aplicaciones web alojadas en Azure Cloud Services. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az webapp list-runtimes -os windows|linux` : lista los entornos de ejecución disponibles para una aplicación web
- `az webapp up -n nombre -l ubicación -r entorno_de_ejecución` : crea una aplicación web
- `az webapp list` : lista todas las aplicaciones web
- `az webapp delete -n nombre -g grupo_de_recursos` : elimina una aplicación web específica

## Links
- More information: https://learn.microsoft.com/cli/azure/webapp
