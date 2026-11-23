# go build

Compila pacchetti e dipendenze.

## Usage
- `go build percorso/del/main.go` : compila un file 'package main' (l'output sarà il nome del file senza estensione)
- `go build -o percorso/del/binary percorso/del/source.go` : compila, specificando il nome del file di output
- `go build -o percorso/del/binary percorso/del/package` : compila un pacchetto
- `go build -race -o percorso/dell/eseguibile percorso/del/pacchetto_main` : compila un pacchetto main in un eseguibile, abilitando il rilevamento race condition

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Compile_packages_and_dependencies
