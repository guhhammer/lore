# apt-clone

Clona/salva/ripristina lo stato dei pacchetti di un sistema basato su Debian.

## Usage
- `apt-clone clone percorso/della/directory` : clona lo stato dei pacchetti del sistema corrente in una directory specificata
- `apt-clone clone --destination path/to/backup.tar.gz` : crea un file clone (`.tar.gz`) per scopi di backup
- `apt-clone restore path/to/backup.tar.gz` : ripristina lo stato dei pacchetti da un file clone
- `apt-clone info path/to/backup.tar.gz` : mostra informazioni su un file clone (es. release, architettura)
- `apt-clone restore path/to/backup.tar.gz --destination path/to/ripristino` : verifica se il file clone può essere ripristinato sul sistema corrente

## Links
- More information: https://github.com/mvo5/apt-clone
