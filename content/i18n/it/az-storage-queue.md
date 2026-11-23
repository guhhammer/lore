# az storage queue

Gestisce le code di archiviazione in Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az storage queue create --account-name nome_account_archiviazione -n nome_coda --metadata metadati_coda` : crea una coda
- `az storage queue generate-sas --account-name nome_account_archiviazione -n nome_coda --permissions permessi_coda --expiry data_scadenza --https-only` : genera una firma di accesso condiviso per la coda
- `az storage queue list --prefix prefisso_filtro --account-name nome_account_archiviazione` : elenca le code in un account di archiviazione
- `az storage queue delete --account-name nome_account_archiviazione -n nome_coda --fail-not-exist` : elimina la coda specificata e tutti i messaggi che contiene

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/queue
