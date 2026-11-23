# az storage

Administra los recursos de Azure Cloud Storage. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az storage account create -g grupo_de_recursos -n nombre_de_cuenta -l ubicación --sku account_sku` : crea una cuenta de almacenamiento
- `az storage account list -g grupo_de_recursos` : enumera todas las cuentas de almacenamiento de un grupo de recursos
- `az storage account keys list -g grupo_de_recursos -n nombre_de_cuenta` : enumera las claves de acceso de una cuenta de almacenamiento
- `az storage account delete -g grupo_de_recursos -n nombre_de_cuenta` : elimina una cuenta de almacenamiento
- `az storage account update --min-tls-version TLS1_0|TLS1_1|TLS1_2 -g grupo_de_recursos -n nombre_de_cuenta` : actualiza la versión mínima de TLS para una cuenta de almacenamiento

## Links
- More information: https://learn.microsoft.com/cli/azure/storage
