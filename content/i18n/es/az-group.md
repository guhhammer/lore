# az group

Administra grupos de recursos e implementaciones de plantillas. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az group create -n nombre -l ubicación` : crea un nuevo grupo de recursos
- `az group exists -n nombre` : comprueba si existe un grupo de recursos
- `az group delete -n nombre` : elimina un grupo de recursos
- `az group wait -n nombre --created|deleted|exists|updated` : coloca un grupo de recursos en estado de espera hasta que se cumpla una condición

## Links
- More information: https://learn.microsoft.com/cli/azure/group
