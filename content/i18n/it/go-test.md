# go test

Testa i pacchetti Go (i file devono terminare con `_test.go`).

## Usage
- `go test` : testa il pacchetto trovato nella directory corrente
- `go test -v` : testa verbose il pacchetto nella directory corrente
- `go test -v ./...` : testa i pacchetti nella directory corrente e tutte le sottodirectory (nota i `...`)
- `go test -v -bench .` : testa il pacchetto nella directory corrente ed esegue tutti i benchmark
- `go test -v -bench . -benchtime 50s` : testa il pacchetto nella directory corrente ed esegue tutti i benchmark per 50 secondi
- `go test -cover` : testa il pacchetto con analisi di copertura

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Testing_flags
