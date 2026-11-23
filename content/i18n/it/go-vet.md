# go vet

Controlla il codice sorgente Go e segnala costruzioni sospette (es. lint dei file sorgente Go). Go vet restituisce un codice di uscita non-zero se trova problemi; restituisce zero se non trova problemi.

## Usage
- `go vet` : controlla il pacchetto Go nella directory corrente
- `go vet percorso/del/file_o_directory` : controlla il pacchetto Go nel percorso specificato
- `go tool vet help` : elenca i controlli disponibili che possono essere eseguiti con go vet
- `go tool vet help check_name` : visualizza dettagli e flag per un controllo particolare
- `go vet -c=n` : mostra le linee offending più `n` linee di contesto circostante
- `go vet -json` : output analisi ed errori in formato JSON

## Links
- More information: https://pkg.go.dev/cmd/vet
