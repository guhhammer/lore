# ab

Strumento di benchmarking per il server HTTP Apache.

## Usage
- `ab -n 100 url` : esegue 100 richieste HTTP GET verso un dato URL
- `ab -n 100 -c 10 url` : esegue 100 richieste HTTP GET in batch concorrenti da 10 verso un URL
- `ab -n 100 -T application/json -p percorso/al/file.json url` : esegue 100 richieste HTTP POST verso un URL usando un payload JSON da file
- `ab -k url` : usa HTTP keep-Alive (più richieste nella stessa sessione HTTP)
- `ab -t 60 url` : imposta il timeout massimo in secondi per il benchmarking (30 di default)
- `ab -e percorso/al/file.csv` : salva i risultati in un file CSV

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
