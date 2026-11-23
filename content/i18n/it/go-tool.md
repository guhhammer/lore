# go tool

Esegui uno strumento o comando Go. Esegue un comando Go come binario standalone, tipicamente per debug.

## Usage
- `go tool` : elenca gli strumenti disponibili
- `go tool link percorso/del/main.go` : esegui lo strumento go link
- `go tool -n comando arguments` : stampa il comando che verrebbe eseguito, senza eseguirlo (simile a `whereis`)
- `go tool comando --help` : visualizza la documentazione per uno strumento specificato
- `go tool dist list` : elenca tutti i target di compilazione cross disponibili

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Run_specified_go_tool
