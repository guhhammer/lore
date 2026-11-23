# az devops

Gestisce le organizzazioni Azure DevOps. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az devops login --org url_organizzazione` : imposta il Personal Access Token (PAT) per accedere a un'organizzazione specifica
- `az devops project show -p nome_progetto --open` : apri un progetto nel browser
- `az devops team list-member -p nome_progetto --team nome_team` : elenca i membri di un team specifico che lavora su un progetto particolare
- `az devops configure -l` : controlla la configurazione corrente della CLI Azure DevOps
- `az devops configure -d project=nome_progetto organization=url_organizzazione` : configura il comportamento della CLI Azure DevOps impostando un progetto predefinito e un'organizzazione predefinita

## Links
- More information: https://learn.microsoft.com/cli/azure/devops
