# az storage blob

Gestisce i container e gli oggetti di archiviazione blob in Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az storage blob download --account-name nome_account --account-key chiave_account -c nome_container -n nome_blob -f percorso/del/file` : scarica un blob in un percorso file specificando un container sorgente
- `az storage blob download-batch --account-name nome_account --account-key chiave_account -s nome_container --pattern regex_nome_file -d percorso/della/destinazione` : scarica blob da un container blob in modo ricorsivo
- `az storage blob upload --account-name nome_account --account-key chiave_account -c nome_container -n nome_blob -f percorso/del/file` : carica un file locale nell'archiviazione blob
- `az storage blob delete --account-name nome_account --account-key chiave_account -c nome_container -n nome_blob` : elimina un oggetto blob
- `az storage blob generate-sas --account-name nome_account --account-key chiave_account -c nome_container -n nome_blob --permissions insieme_permessi --expiry Y-m-d'T'H:M'Z' --https-only` : genera una firma di accesso condiviso per un blob

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/blob
