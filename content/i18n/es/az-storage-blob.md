# az storage blob

Administra contenedores y objetos de almacenamiento de blobs en Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az storage blob download --account-name nombre_cuenta --account-key llave_cuenta -c nombre_contenedor -n nombre_blob -f ruta/al/archivo` : descarga un blob a una ruta de acceso de archivo especificando un contenedor de origen
- `az storage blob download-batch --account-name nombre_cuenta --account-key llave_cuenta -s nombre_contenedor --pattern nombre_archivo_regex -d ruta/al/destino` : descarga blobs desde un contenedor de blobs recursivamente
- `az storage blob upload --account-name nombre_cuenta --account-key llave_cuenta -c nombre_contenedor -n nombre_blob -f ruta/al/archivo` : sube un archivo local al almacenamiento de blobs
- `az storage blob delete --account-name nombre_cuenta --account-key llave_cuenta -c nombre_contenedor -n nombre_blob` : elimina un objeto blob
- `az storage blob generate-sas --account-name nombre_cuenta --account-key llave_cuenta -c nombre_contenedor -n nombre_blob --permissions grupo_permisos --expiry Y-m-d'T'H:M'Z' --https-only` : genera una firma de acceso compartido para un blob

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/blob
