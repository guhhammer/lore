# conda clean

Elimina file temporanei o non utilizzati: cache degli indici, file di blocco, pacchetti inutilizzati, tarball e file di log.

## Usage
- `conda clean -avy` : elimina tutti i file temporanei o non utilizzati in modo dettagliato e conferma automaticamente tutte le richieste
- `conda clean -itl` : elimina solo la cache degli indici, i tarball e i file di log
- `conda clean -c percorso/a/file_temporanei` : elimina solo i file temporanei di cache che non potevano essere eliminati in precedenza perché in uso
- `conda clean -p` : elimina solo i pacchetti non utilizzati. Potrebbe eliminare pacchetti installati con softlink
- `conda clean -f` : forza l'eliminazione di tutti i pacchetti scrivibili. È più esteso dell'opzione `--all`. Eliminerà anche i pacchetti installati con softlink
- `conda clean -h` : mostra l'aiuto

## Links
- More information: https://docs.conda.io/projects/conda/en/stable/commands/clean.html
