# az storage account

Administra cuentas de almacenamiento en Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az storage account create -n nombre_cuenta_almacenamiento -g grupo_recurso_azure --location ubicación_azure --sku cuenta_almacenamiento_sku` : crea una cuenta de almacenamiento
- `az storage account generate-sas --account-name nombre_cuenta_almacenamiento -n nombre_cuenta --permissions permisos_sas --expiry fecha_caducidad --services servicios_almacenamiento --resource-types tipos_recursos` : genera una firma de acceso compartido para una cuenta de almacenamiento específica
- `az storage account list -g grupo_recurso_azure` : lista cuentas de almacenamiento
- `az storage account delete -n nombre_cuenta_almacenamiento -g grupo_recurso_azure` : elimina una cuenta de almacenamiento específica

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/account
