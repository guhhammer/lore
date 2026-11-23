# az apim

Administra los servicios de Azure API Management. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az apim list -g grupo_de_recursos` : enumera las instancias del servicio API Management
- `az apim create -n nombre -g grupo_de_recursos --publisher-email email --publisher-name name` : crea una instancia de servicio de API Management
- `az apim delete -n nombre -g grupo_de_recursos` : elimina una instancia del servicio de API Management
- `az apim show -n nombre -g grupo_de_recursos` : muestra detalles de una instancia del servicio de API Management
- `az apim update -n nombre -g grupo_de_recursos` : actualiza una instancia del servicio API Management

## Links
- More information: https://learn.microsoft.com/cli/azure/apim
