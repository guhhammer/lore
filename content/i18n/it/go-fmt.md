# go fmt

Formatta i sorgenti Go, stampando i nomi dei file modificati.

## Usage
- `go fmt` : formatta i sorgenti Go nella directory corrente
- `go fmt percorso/del/pacchetto` : formatta un pacchetto Go specifico nel percorso di import (`$GOPATH/src`)
- `go fmt ./...` : formatta il pacchetto nella directory corrente e tutte le sottodirectory (nota i `...`)
- `go fmt -n` : stampa i comandi di formattazione che sarebbero eseguiti, senza modificare nulla
- `go fmt -x` : stampa i comandi di formattazione mentre vengono eseguiti

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Gofmt__reformat__package_sources
