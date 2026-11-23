# pip install

Installa pacchetti Python.

## Usage
- `pip install pacchetto1 pacchetto2 ...` : installa uno o più pacchetti
- `pip install pacchetto1 pacchetto2 ... -U` : aggiorna tutti i pacchetti specificati all'ultima versione, installando anche quelli non ancora presenti
- `pip install pacchetto==versione` : installa una versione specifica di un pacchetto
- `pip install -r percorso/del/requirements.txt` : installa i pacchetti elencati in un file
- `pip install percorso/del/file.whl|percorso/del/file.tar.gz|percorso/della/directory` : installa un pacchetto da un archivio locale o da una directory
- `pip install git+https://example.com/nome_utente/repository.git` : installa un pacchetto da un repository Git
- `pip install -f url|percorso/della/directory pacchetto` : installa un pacchetto da una sorgente alternativa (URL o directory) invece che da PyPI
- `pip install -e .` : installa il pacchetto locale nella directory corrente in modalità sviluppo (modificabile)

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_install/
