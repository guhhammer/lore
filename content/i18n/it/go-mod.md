# go mod

Manutenzione moduli.

## Usage
- `go mod init nome_modulo` : inizializza nuovo modulo nella directory corrente
- `go mod download` : scarica moduli nella cache locale
- `go mod tidy` : aggiungi moduli mancanti e rimuovi quelli inutilizzati
- `go mod verify` : verifica che le dipendenze abbiano il contenuto atteso
- `go mod vendor` : copia le sorgenti di tutte le dipendenze nella directory vendor

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Module_maintenance
