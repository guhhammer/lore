# az account

Administra la información de una suscripción de Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az account list` : lista las suscripciones de la cuenta activa
- `az account set -s identificador_de_suscripción` : establece una `subscription` como la suscripción activa
- `az account list-locations` : lista las regiones admitidas para la suscripción activa
- `az account get-access-token --resource-type ms-graph` : imprime un token de acceso para usar con la `MS Graph API`
- `az account show -o json|tsv|table|yaml` : imprime los detalles de la suscripción activa actual en un formato específico

## Links
- More information: https://learn.microsoft.com/cli/azure/account
