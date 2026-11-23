# choco install

Installa uno o più pacchetti con Chocolatey.

## Usage
- `choco install pacchetto1 pacchetto2 ...` : installa uno o più pacchetti separati da spazio
- `choco install percorso\del\file_di_pacchetti.config` : installa pacchetti da un file di configurazione personalizzato
- `choco install percorso\del\file` : installa un file `.nuspec` o `.nupkg` specifico
- `choco install pacchetto --version versione` : installa una nuova versione specifica di un pacchetto
- `choco install pacchetto --allow-multiple` : consenti l'installazione di più versioni di un pacchetto
- `choco install pacchetto -y` : conferma automaticamente tutte le richieste
- `choco install pacchetto -s url_fonte|alias` : specifica una fonte personalizzata per ricevere pacchetti
- `choco install pacchetto -u nome_utente -p password` : fornisci un nome utente e una password per l'autenticazione

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
