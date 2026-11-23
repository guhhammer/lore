# az storage entity

Gestisce le entità di archiviazione tabelle Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az storage entity insert -e coppie_chiave_valore_separate_dallo_spazio -t nome_tabella --account-name nome_account_archiviazione --account-key chiave_account_archiviazione` : inserisce un'entità in una tabella
- `az storage entity delete --partition-key chiave_partizione --row-key chiave_riga -t nome_tabella --account-name nome_account_archiviazione --account-key chiave_account_archiviazione` : elimina un'entità esistente da una tabella
- `az storage entity merge -e coppie_chiave_valore_separate_dallo_spazio -t nome_tabella --account-name nome_account_archiviazione --account-key chiave_account_archiviazione` : aggiorna un'entità esistente unendo le sue proprietà
- `az storage entity query --filter filtro_query -t nome_tabella --account-name nome_account_archiviazione --account-key chiave_account_archiviazione` : elenca le entità che soddisfano una query
- `az storage entity show --partition-key chiave_partizione --row-key chiave_riga -t nome_tabella --account-name nome_account_archiviazione --account-key chiave_account_archiviazione` : ottiene un'entità dalla tabella specificata

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/entity
