# az provider

Gestisce i provider di risorse. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az provider register -n Microsoft.PolicyInsights` : registra un provider
- `az provider unregister -n Microsoft.Automation` : annulla la registrazione di un provider
- `az provider list` : elenca tutti i provider per una sottoscrizione
- `az provider show -n Microsoft.Storage` : mostra informazioni su un provider specifico
- `az provider list --query "[?namespace=='Microsoft.Network'].resourceTypes[].resourceType"` : elenca tutti i tipi di risorse per un provider specifico

## Links
- More information: https://learn.microsoft.com/cli/azure/provider
