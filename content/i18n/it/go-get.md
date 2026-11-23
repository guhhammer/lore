# go get

Aggiunge un pacchetto di dipendenza o scarica pacchetti in modalità GOPATH legacy.

## Usage
- `go get example.com/pkg` : aggiunge il pacchetto specificato a `go.mod` in modalità modulo o installa il pacchetto in modalità GOPATH
- `go get example.com/pkg@v1.2.3` : modifica il pacchetto con una versione specifica in modalità module-aware
- `go get example.com/pkg@none` : rimuove un pacchetto specificato
