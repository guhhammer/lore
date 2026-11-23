# ag

The Silver Searcher. Come `ack`, ma più veloce.

## Usage
- `ag foo` : trova file contenenti "foo" e mostra le corrisponenti linee contenenti il termine
- `ag foo percorso/della/directory` : trova file contenenti "foo" in una specifica directory
- `ag -l foo` : trova file contenenti "foo" elencandone solamente i nomi
- `ag -i -o FOO` : trova file contenenti "FOO" senza distinguere tra maiuscole e minuscole, e stampa solo il termine trovato piuttosto che l'intera linea
- `ag foo -G bar` : trova "foo" in file il quale nome contiene "bar"
- `ag 'espressione_regolare'` : trova file il quale contenuto soddisfi una determinata espressione regolare
- `ag -g foo` : trova file il quale nome contiene "foo"

## Links
- More information: https://manned.org/ag
