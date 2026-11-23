# az image

Administra imagenes de máquinas virtuales personalizadas en Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az image list -g grupo_recursos` : lista imagenes personalizadas en un grupo de recursos
- `az image create -g grupo_recursos -n nombre --os-type windows|linux --source origen_disco_sistema_operativo` : crea una imagen personalizada desde un disco existente o snapshots
- `az image delete -n nombre -g grupo_recursos` : elimina una imagen personalizada
- `az image show -n nombre -g grupo_recursos` : muestra detalles de una imagen personalizada
- `az image update -n nombre -g grupo_recursos --set propiedad=valor` : actualiza imagenes personalizadas

## Links
- More information: https://learn.microsoft.com/cli/azure/image
