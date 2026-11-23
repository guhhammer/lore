# jira

Interfaccia (terze parti) per interagire con Jira. Nota: È necessario ottenere un token API Jira ed esportarlo nella shell come variabile `$JIRA_API_TOKEN`.

## Usage
- `jira init` : crea un file di configurazione (richiesto prima di usare `jira`)
- `jira issue ls` : elenca le issues recenti
- `jira issue ls -a x -y High` : elenca issues non assegnate con alta priorità
- `jira sprint ls --current -a $(jira me)` : elenca issues dallo sprint corrente, assegnate a me
- `jira issue create -P parent` : crea una nuova issue con issue genitore
- `jira open 123` : apri un'issue nel browser

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#commands
