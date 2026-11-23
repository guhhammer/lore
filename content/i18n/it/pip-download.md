# pip download

Scarica pacchetti Python senza installarli.

## Usage
- `pip download pacchetto` : scarica un file wheel o un archivio sorgente di un pacchetto nella directory corrente
- `pip download pacchetto==versione` : scarica una versione specifica di un pacchetto
- `pip download pacchetto -d percorso/della/directory` : scarica un pacchetto e le sue dipendenze in una directory specifica
- `pip download pacchetto --only-binary :all: --platform piattaforma --python-version versione` : scarica un pacchetto per una specifica piattaforma e versione di Python
- `pip download pacchetto -i url` : scarica un pacchetto da un URL di indice specifico

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_download/
