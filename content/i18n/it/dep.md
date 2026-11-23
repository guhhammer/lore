# dep

Strumento di gestione delle dipendenze per progetti Go.

## Usage
- `dep init` : inizializza la directory corrente come radice di un progetto Go
- `dep ensure` : installa dipendenze mancanti (scannerizza `Gopkg.toml` ed i file `.go`)
- `dep status` : mostra lo stato delle dipendenze di un progetto
- `dep ensure -add url_pacchetto` : aggiungi una dipendenza al progetto
- `dep ensure -update` : aggiorna le versioni bloccate (in `Gopkg.lock`) di tutte le dipendenze

## Links
- More information: https://deployer.org/docs/8.x/cli
