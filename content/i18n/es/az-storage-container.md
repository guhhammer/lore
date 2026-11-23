# az storage container

Administra contenedores de almacenamiento de blobs en Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az storage container create --account-name nombre_cuenta_almacenamiento -n nombre_contenedor --public-access nivel_acceso --fail-on-exist` : crea un contenedor en una cuenta de almacenamiento
- `az storage container generate-sas --account-name nombre_cuenta_almacenamiento -n nombre_contenedor --permissions permisos_sas --expiry fecha_caducidad --https-only` : genera una firma de acceso compartido para el contenedor
- `az storage container list --account-name nombre_cuenta_almacenamiento --prefix filtrar_prefijo` : lista los contenedores en una cuenta de almacenamiento
- `az storage container delete --account-name nombre_cuenta_almacenamiento -n nombre_contenedor --fail-not-exist` : marca el contenedor especificado para su eliminación

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/container
