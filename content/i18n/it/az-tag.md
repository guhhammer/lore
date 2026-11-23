# az tag

Gestisce i tag su una risorsa. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az tag add-value -n nome_tag --value valore_tag` : crea un valore di tag
- `az tag create -n nome_tag` : crea un tag nella sottoscrizione
- `az tag delete -n nome_tag` : elimina un tag dalla sottoscrizione
- `az tag list --resource-id /subscriptions/id_sottoscrizione` : elenca tutti i tag su una sottoscrizione
- `az tag remove-value -n nome_tag --value valore_tag` : elimina un valore di tag per un nome di tag specifico

## Links
- More information: https://learn.microsoft.com/cli/azure/tag
