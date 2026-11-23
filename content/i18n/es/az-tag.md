# az tag

Administra etiquetas en un recurso de Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az tag add-value -n nombre_de_etiqueta --value valor_de_etiqueta` : crea un valor de etiqueta
- `az tag create -n nombre_de_etiqueta` : crea una etiqueta en la suscripción
- `az tag delete -n nombre_de_etiqueta` : elimina una etiqueta de la suscripción
- `az tag list --resource-id /subscriptions/identificador_de_subscripción` : enumera todas las etiquetas de una suscripción
- `az tag remove-value -n nombre_de_etiqueta --value valor_de_etiqueta` : elimina un valor de etiqueta para un nombre de etiqueta específico

## Links
- More information: https://learn.microsoft.com/cli/azure/tag
