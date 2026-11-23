# az appconfig

Administra las configuraciones de aplicaciones en Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az appconfig create -n nombre -g grupo_de_recursos -l ubicación` : crea una configuración de aplicación
- `az appconfig delete -g grupo_de_recursos -n nombre_de_configuración` : elimina una configuración de aplicación específica
- `az appconfig list` : lista todas las configuraciones de aplicaciones bajo la suscripción actual
- `az appconfig list -g grupo_de_recursos` : lista todas las configuraciones de aplicaciones bajo un grupo de recursos específico
- `az appconfig show -n nombre_de_configuración` : muestra las propiedades de una configuración de aplicación
- `az appconfig update -g grupo_de_recursos -n nombre_de_configuración` : actualiza una configuración de aplicación específica

## Links
- More information: https://learn.microsoft.com/cli/azure/appconfig
