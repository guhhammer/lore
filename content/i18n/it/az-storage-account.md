# az storage account

Gestisce gli account di archiviazione in Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az storage account create -n nome_account_archiviazione -g gruppo_risorse_azure --location posizione_azure --sku sku_account_archiviazione` : crea un account di archiviazione
- `az storage account generate-sas --account-name nome_account_archiviazione -n nome_account --permissions permessi_sas --expiry data_scadenza --services servizi_archiviazione --resource-types tipi_risorse` : genera una firma di accesso condiviso per un account di archiviazione specifico
- `az storage account list -g gruppo_risorse_azure` : elenca gli account di archiviazione
- `az storage account delete -n nome_account_archiviazione -g gruppo_risorse_azure` : elimina un account di archiviazione specifico

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/account
