# curl

Trasferisci dati da o ad un server. Supporta la maggior parte dei protocolli, tra cui HTTP, HTTPS, FTP, SCP, etc.

## Usage
- `curl https://example.com` : esegue una richiesta HTTP GET e stampa il contenuto in `stdout`
- `curl -L -D - https://example.com` : esegue una richiesta HTTP GET, L segue eventuali reindirizzamenti `3xx` e D stampa il contenuto e la intestazione della risposta su `stdout`
- `curl -O https://example.com/nome_file.zip` : scarica un file salvando l'Output con lo stesso nome indicato nell'URL
- `curl -X POST -d 'name=mario' http://example.com/form` : invia dati form-encoded (con una richiesta POST di tipo `application/x-www-form-urlencoded`). Utilizza `--data @file_name` oppure `--data @'-'` per leggere da `stdin`
- `curl -k -x http://127.0.0.1:8080 -H 'Authorization: Bearer token' -X GET|PUT|POST|DELETE|PATCH|... https://example.com` : invia una richiesta con una intestazione aggiuntiva utilizzando un metodo HTTP personalizzato attraverso un proxy (come BurpSuite) ignorando i certificati autofirmati non sicuri
- `curl -d '{"name":"mario"}' -H 'Content-Type: application/json' http://example.com/users/1234` : invia dati in formato JSON, specificando la H intestazione content-type appropriata
- `curl -E cliente.pem --key chiave.pem -k https://example.com` : invia il certificato client e la chiave per una risorsa omettendo la validazione del certificato
- `curl -v --resolve example.com:80:127.0.0.1 http://example.com` : risolve un hostname con un indirizzo IP personalizzato con un output v dettagliato (simile alla modifica del file `/etc/hosts` per la risoluzione di un DNS personalizzato)

## Links
- More information: https://curl.se/docs/manpage.html
