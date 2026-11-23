# az storage

Gestisce le risorse di Archiviazione cloud di Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az storage account create -g nome_gruppo -n nome_account -l posizione --sku sku_account` : crea un account di archiviazione specificando una posizione
- `az storage account list -g nome_gruppo` : elenca tutti gli account di archiviazione in un gruppo di risorse
- `az storage account keys list -g nome_gruppo -n nome_account` : elenca le chiavi di accesso per un account di archiviazione
- `az storage account delete -g nome_gruppo -n nome_account` : elimina un account di archiviazione
- `az storage account update --min-tls-version TLS1_0|TLS1_1|TLS1_2 -g nome_gruppo -n nome_account` : aggiorna l'impostazione della versione minima TLS per un account di archiviazione

## Links
- More information: https://learn.microsoft.com/cli/azure/storage
