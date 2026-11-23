# go install

Compila e installa i pacchetti nominati dai percorsi di import.

## Usage
- `go install` : compila e installa il pacchetto corrente
- `go install percorso/del/pacchetto` : compila e installa un pacchetto locale specifico
- `go install golang.org/x/tools/gopls@latest` : installa l'ultima versione di un programma, ignorando `go.mod` nella directory corrente
- `go install golang.org/x/tools/gopls` : installa un programma alla versione selezionata da `go.mod` nella directory corrente

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Compile_and_install_packages_and_dependencies
