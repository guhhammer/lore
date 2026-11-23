# ed

L'originale editor di testo Unix.

## Usage
- `ed` : avvia ed per editare un documento vuoto (che può essere salvato come nuovo file nella directory corrente)
- `ed -p :` : avvia ed per editare un documento vuoto, con `:` come indicatore del prompt di comandi
- `ed -p : percorso/del/file` : avvia ed per editare un file esistente (mostra il numero di byte del file caricato)
- `H` : attiva o disattiva la stampa di spiegazioni per gli errori (di default, le spiegazioni non sono stampate ed appare solo un `?`)
- `a<Enter>text_to_insert<Enter>.` : aggiungi del testo al documento corrente. Indica il completamento inserendo un punto da solo su una nuova linea
- `,p` : stampa l'intero documento (`,` è una scorciatoia per il range `1,$` che copre dall'inizio alla fine del documento)
- `w nome_file` : scrivi il documento corrente su un nuovo file (il nome del file può essere omesso se `ed` è stato avviato con un file esistente)
- `q<Enter>` : termina ed

## Links
- More information: https://www.gnu.org/software/ed/manual/ed_manual.html
