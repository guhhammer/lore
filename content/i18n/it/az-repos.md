# az repos

Gestisce i repository Azure DevOps. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az repos list -p nome_progetto` : elenca tutti i repository in un progetto specifico
- `az repos policy merge-strategy create --repository-id id_repository_da_elenco_repos --branch nome_branch --blocking --enabled --allow-no-fast-forward false --allow-rebase true --allow-rebase-merge true --allow-squash true` : aggiunge una policy su un branch specifico di un repository specifico per restringere la merge base
- `az repos policy build create --repository-id id_repository --build-definition-id id_pipeline_build --branch main --blocking --enabled --queue-on-source-update-only true --display-name nome --valid-duration minuti` : aggiunge una validazione build su un repository specifico, usando una pipeline build esistente, da attivare automaticamente su aggiornamento sorgente
- `az repos pr list -p nome_progetto -r nome_repository --status active` : elenca tutte le Pull Request attive su un repository specifico all'interno di un progetto specifico

## Links
- More information: https://learn.microsoft.com/cli/azure/repos
