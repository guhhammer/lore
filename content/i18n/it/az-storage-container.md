# az storage container

Gestisce i container di archiviazione blob in Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az storage container create --account-name nome_account_archiviazione -n nome_container --public-access livello_accesso --fail-on-exist` : crea un container in un account di archiviazione
- `az storage container generate-sas --account-name nome_account_archiviazione -n nome_container --permissions permessi_sas --expiry data_scadenza --https-only` : genera una firma di accesso condiviso per il container
- `az storage container list --account-name nome_account_archiviazione --prefix prefisso_filtro` : elenca i container in un account di archiviazione
- `az storage container delete --account-name nome_account_archiviazione -n nome_container --fail-not-exist` : segna il container specificato per l'eliminazione

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/container
