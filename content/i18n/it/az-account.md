# az account

Gestisce le informazioni di sottoscrizione Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az account list` : elenca tutte le sottoscrizioni per l'account loggato
- `az account set -s id_sottoscrizione` : imposta una `sottoscrizione` come quella attualmente attiva
- `az account list-locations` : elenca le regioni supportate per la sottoscrizione attualmente attiva
- `az account get-access-token --resource-type ms-graph` : stampa un token di accesso da usare con `MS Graph API`
- `az account show -o json|tsv|table|yaml` : stampa i dettagli della sottoscrizione attualmente attiva in un formato specifico

## Links
- More information: https://learn.microsoft.com/cli/azure/account
