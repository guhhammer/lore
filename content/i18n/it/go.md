# go

Gestisci il codice sorgente Go. Alcuni sottocomandi come `build` hanno la propria documentazione d'uso.

## Usage
- `go get percorso/del/pacchetto` : scarica e installa un pacchetto, specificato tramite il suo import path
- `go run percorso/del/file.go` : compila ed esegui un file sorgente (deve contenere un package `main`)
- `go build -o eseguibile percorso/del/file.go` : compila un file sorgente in un eseguibile con nome specifico
- `go build` : compila il pacchetto presente nella directory corrente
- `go test` : esegui tutti i test del pacchetto corrente (i file devono terminare con `_test.go`)
- `go install` : compila e installa il pacchetto corrente
- `go mod init nome_modulo` : inizializza un nuovo modulo nella directory corrente

## Links
- More information: https://pkg.go.dev/cmd/go
