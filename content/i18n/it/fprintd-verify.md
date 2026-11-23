# fprintd-verify

Verifica le impronte digitali rispetto al database.

## Usage
- `fprintd-verify` : verifica tutte le impronte digitali memorizzate per l'utente corrente
- `fprintd-verify -f left-thumb|left-index-finger|left-middle-finger|left-ring-finger|left-little-finger|right-thumb|...` : verifica un'impronta digitale specifica per l'utente corrente
- `fprintd-verify nome_utente` : verifica le impronte digitali per un utente specifico
- `fprintd-verify -f finger_name nome_utente` : verifica un'impronta digitale specifica per un utente specifico
- `fprintd-verify --g-fatal-warnings` : fallisce il processo se un'impronta digitale non corrisponde a quelle memorizzate nel database per l'utente corrente
- `fprintd-verify -h` : mostra l'aiuto

## Links
- More information: https://manned.org/fprintd-verify
