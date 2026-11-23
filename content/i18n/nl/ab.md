# ab

Apache HTTP-serverbenchmarktool.

## Usage
- `ab -n 100 url` : voer 100 HTTP GET-verzoeken uit naar een bepaalde URL
- `ab -n 100 -c 10 url` : voer 100 HTTP GET-verzoeken uit, in gelijktijdige batches van 10, naar een URL
- `ab -n 100 -T application/json -p pad/naar/bestand.json url` : voer 100 HTTP POST-verzoeken uit naar een URL, met behulp van een JSON-payload uit een bestand
- `ab -k url` : gebruik HTTP keep-Alive, d.w.z. voer meerdere verzoeken uit binnen één HTTP-sessie
- `ab -t 60 url` : stel het maximale aantal seconden (timeout) in dat je wilt besteden aan benchmarking (standaard 30)
- `ab -e pad/naar/bestand.csv` : schrijf de resultaten naar een CSV-bestand

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
