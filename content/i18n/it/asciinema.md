# asciinema

Registra e riproduci sessioni di terminale, condividendole opzionalmente su asciiname.org.

## Usage
- `asciinema auth` : associa l'installazione locale di `asciiname` ad un account di asciiname.org
- `asciinema rec` : avvia una nuova registrazione (una volta terminata, verrà chiesto se caricarla o salvarla localmente)
- `asciinema rec percorso/del/registrazione.cast` : avvia una nuova registrazione e salvala come file locale
- `asciinema play percorso/del/registrazione.cast` : riproduci una sessione da un file locale
- `asciinema play https://asciinema.org/a/id_registrazione` : riproducei una sessione da asciinema.org
- `asciinema rec -i 2.5` : avvia una nuova registrazione, limitando qualsiasi periodo di inattività a 2.5 secondi
- `asciinema cat percorso/del/registrazione.cast` : stampa l'output completo di una sessione locale
- `asciinema upload percorso/del/registrazione.cast` : carica una sessione locale su asciinama.org

## Links
- More information: https://docs.asciinema.org/manual/cli/
