# aws configure

Gestisce la configurazione per AWS CLI.

## Usage
- `aws configure` : configura AWS CLI in modo interattivo (crea una nuova configurazione o aggiorna quella predefinita)
- `aws configure --profile nome_profilo` : configura un profilo nominato per AWS CLI in modo interattivo (crea un nuovo profilo o aggiorna quello esistente)
- `aws configure get nome` : visualizza il valore da una variabile di configurazione specifica
- `aws configure get nome --profile nome_profilo` : visualizza il valore per una variabile di configurazione in un profilo specifico
- `aws configure set nome valore` : imposta il valore di una variabile di configurazione specifica
- `aws configure set nome valore --profile nome_profilo` : imposta il valore di una variabile di configurazione in un profilo specifico
- `aws configure list` : elenca le voci di configurazione
- `aws configure list --profile nome_profilo` : elenca le voci di configurazione per un profilo specifico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/configure/
