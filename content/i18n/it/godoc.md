# godoc

Visualizza la documentazione per i pacchetti Go.

## Usage
- `godoc fmt` : mostra l'aiuto per un pacchetto specifico
- `godoc fmt Printf` : mostra l'aiuto per la funzione "Printf" del pacchetto "fmt"
- `godoc -http=:6060` : serve la documentazione come web server sulla porta 6060
- `godoc -write_index -index_files=percorso/del/file` : crea un file indice
- `godoc -http=:6060 -index -index_files=percorso/del/file` : usa il file indice specificato per cercare nella documentazione

## Links
- More information: https://pkg.go.dev/golang.org/x/tools/cmd/godoc
