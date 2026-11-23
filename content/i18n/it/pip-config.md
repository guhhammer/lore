# pip config

Gestisci la configurazione locale e globale per pip.

## Usage
- `pip config list` : elenca tutti i valori di configurazione
- `pip config debug` : mostra i file di configurazione e i loro valori
- `pip config set command.option valore --global|--user|--site` : imposta il valore per un'opzione di comando
- `pip config get command.option --global|--user|--site` : ottieni il valore per un'opzione di comando
- `pip config unset command.option --global|--user|--site` : rimuovi il valore impostato per un'opzione di comando
- `pip config edit --global|--user|--site` : modifica il file di configurazione con l'editor predefinito
- `pip config edit --global|--user|--site --editor percorso/del/binario_editor` : modifica il file di configurazione con uno specifico editor

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_config/
