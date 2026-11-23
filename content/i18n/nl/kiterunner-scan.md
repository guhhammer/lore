# kiterunner scan

Een contextuele webscanner om gelijktijdig API-paden en webeindpunten te scannen met behulp van kitebuilder woordenlijsten. Het `scan` subcommando richt zich op een of meerdere hosts met gestructureerde API-verzoeken.

## Usage
- `kiterunner scan https://example.com -A apiroutes-210228:5000` : scan een doel met een Assetnote woordenlijst (bijvoorbeeld de eerste 5000 API-routes)
- `kiterunner scan https://example.com -w pad/naar/woordenlijst.kite` : scan een doel met een kitebuilder woordenlijst
- `kiterunner scan pad/naar/hosts.txt -w pad/naar/woordenlijst.kite` : scan meerdere hosts vanuit een bestand met een kitebuilder woordenlijst
- `kiterunner scan https://example.com -A apiroutes-210228:5000 -o json` : scan met een Assetnote woordenlijst en JSON-uitvoer
- `kiterunner scan https://example.com -w pad/naar/woordenlijst.kite -x 5 -j 100` : scan met aangepaste concurrency-instellingen voor prestaties
- `kiterunner scan https://example.com -w pad/naar/rafter.txt -d 0` : scan met een woordenlijst als een normale woordenlijst, waarbij het scannen op diepte wordt uitgeschakeld
- `kiterunner scan https://example.com -w pad/naar/woordenlijst.kite -H "Authorization: Bearer token" --ignore-length 100-105` : scan met aangepaste headers en negeer antwoorden met specifieke inhoudslengte
- `kiterunner scan https://example.com -w pad/naar/woordenlijst.kite --kitebuilder-full-scan` : voer een volledige kitebuilder scan uit zonder fasescanning

## Links
- More information: https://github.com/assetnote/kiterunner#usage
