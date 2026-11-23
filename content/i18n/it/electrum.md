# electrum

Ergonomico wallet (portafogli) Bitcoin e gestore di chiavi private.

## Usage
- `electrum -w nuovo_wallet.dat create` : crea un nuovo wallet
- `electrum -w wallet_ripristinato.dat restore -o` : ripristina un wallet esistente da un seed offline
- `electrum mktx destinatario ammontare -f 0.0000001 -F mittente -o` : crea una transazione firmata offline
- `electrum listaddresses -a` : mostra tutti gli indirizzi del wallet per la ricezione
- `electrum signmessage indirizzo messaggio` : firma un messaggio
- `electrum verifymessage indirizzo firma messaggio` : verifica un messaggio
- `electrum -p socks5:127.0.0.1:9050 -s 56ckl5obj37gypcu.onion:50001:t -1` : connettiti solo ad una specifica istanza electrum-server

## Links
- More information: https://manned.org/electrum
