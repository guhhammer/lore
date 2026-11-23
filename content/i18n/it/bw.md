# bw

CLI per accedere e gestire vault Bitwarden.

## Usage
- `bw login` : esegui il login ad un account Bitwarden
- `bw logout` : esegui il logout da un account Bitwarden
- `bw list items --search github` : cerca e mostra oggetti in un vault Bitwarden
- `bw get item github` : mostra un particolare oggetto contenuto in un vault Bitwarden
- `echo -n '{"name":"Nome directory"}' | base64 | bw create folder` : crea una directory in un vault bitwarden

## Links
- More information: https://bitwarden.com/help/cli/
