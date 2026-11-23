# az storage table

Gestisce l'archiviazione NoSQL chiave-valore in Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az storage table create --account-name nome_account_archiviazione -n nome_tabella --fail-on-exist` : crea una nuova tabella nell'account di archiviazione
- `az storage table generate-sas --account-name nome_account_archiviazione -n nome_tabella --permissions permessi_sas --expiry data_scadenza --https-only` : genera una firma di accesso condiviso per la tabella
- `az storage table list --account-name nome_account_archiviazione` : elenca le tabelle in un account di archiviazione
- `az storage table delete --account-name nome_account_archiviazione -n nome_tabella --fail-not-exist` : elimina la tabella specificata e tutti i dati che contiene

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/table
