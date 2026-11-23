# ack

Un tool di ricerca simile a `grep`, ottimizzato per programmatori.

## Usage
- `ack "pattern_di_ricerca"` : cerca ricorsivamente file contenenti una stringa o un'espressione regolare nella directory corrente
- `ack -i "pattern_di_ricerca"` : cerca un pattern in modalità case-insensitive
- `ack -o "pattern_di_ricerca"` : cerca righe di testo contenenti un pattern, mostrando solo il testo corrispondente e non il resto della riga
- `ack -t ruby "pattern_di_ricerca"` : limita la ricerca ai file di un tipo specifico
- `ack -t noruby "pattern_di_ricerca"` : non cercare nei file di un tipo specifico
- `ack -c -h "pattern_di_ricerca"` : conta il numero totale di corrispondenze trovate
- `ack -c -l "pattern_di_ricerca"` : mostra i nomi dei file e il numero di corrispondenze per ogni singolo file
- `ack --help-types` : mostra la lista di tutti i valori che possono essere usati con `--type`

## Links
- More information: https://beyondgrep.com/documentation/
