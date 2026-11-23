# az pipelines

Gestisce le risorse Azure Pipelines. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az pipelines create --org url_organizzazione -p nome_progetto --name nome_pipeline --description descrizione --repository nome_repository --branch nome_branch` : crea una nuova pipeline Azure (basata su YAML)
- `az pipelines delete --org url_organizzazione -p nome_progetto --id id_pipeline` : elimina una pipeline specifica
- `az pipelines list --org url_organizzazione -p nome_progetto` : elenca le pipeline
- `az pipelines run --org url_organizzazione -p nome_progetto --name nome_pipeline` : metti in coda una pipeline specifica per l'esecuzione
- `az pipelines show --org url_organizzazione -p nome_progetto --name nome_pipeline` : ottiene i dettagli di una pipeline specifica
- `az pipelines update --org url_organizzazione -p nome_progetto --name nome_pipeline --new-name nome_pipeline_nuovo --new-folder-path user1/production_pipelines` : aggiorna una pipeline specifica
- `az pipelines agent list --org url_organizzazione --pool-id pool_agenti` : elenca tutti gli agenti in un pool

## Links
- More information: https://learn.microsoft.com/cli/azure/pipelines
