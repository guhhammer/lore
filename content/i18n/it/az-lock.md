# az lock

Gestisce i blocchi Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az lock create -n nome_blocco -t ReadOnly` : crea un blocco di livello sottoscrizione in sola lettura
- `az lock create -n nome_blocco -g nome_gruppo -t ReadOnly` : crea un blocco di livello gruppo di risorse in sola lettura
- `az lock delete -n nome_blocco` : elimina un blocco di livello sottoscrizione
- `az lock delete -n nome_blocco -g nome_gruppo` : elimina un blocco di livello gruppo di risorse
- `az lock list` : elenca tutti i blocchi a livello sottoscrizione
- `az lock show -n nome_blocco` : mostra un blocco di livello sottoscrizione con un nome specifico

## Links
- More information: https://learn.microsoft.com/cli/azure/lock
