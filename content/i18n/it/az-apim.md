# az apim

Gestisce i servizi Azure API Management. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az apim list -g gruppo_risorse` : elenca i servizi API Management all'interno di un gruppo di risorse
- `az apim create -n nome -g gruppo_risorse --publisher-email email --publisher-name nome` : crea un'istanza di servizio API Management
- `az apim delete -n nome -g gruppo_risorse` : elimina un servizio API Management
- `az apim show -n nome -g gruppo_risorse` : mostra i dettagli di un'istanza di servizio API Management
- `az apim update -n nome -g gruppo_risorse` : aggiorna un'istanza di servizio API Management

## Links
- More information: https://learn.microsoft.com/cli/azure/apim
