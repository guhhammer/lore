# kiterunner brute

Een contextuele webscanner voor het bruteforcen van API-paden en webeindpunten met behulp van woordenlijsten. Het `brute` subcommando richt zich op een of meerdere hosts.

## Usage
- `kiterunner brute https://example.com -A apiroutes-210328:20000` : bruteforce een doel met een Assetnote woordenlijst (bijvoorbeeld de eerste 20.000 API-routes)
- `kiterunner brute https://example.com -w pad/naar/woordenlijst.txt` : bruteforce een doelwit met een aangepaste woordenlijst
- `kiterunner brute https://example.com -w pad/naar/dirsearch.txt -D -e json,txt` : bruteforce met een dirsearch-stijl woordenlijst met extensie-substitutie
- `kiterunner brute https://example.com -w pad/naar/woordenlijst.txt -e aspx,ashx -o json` : bruteforce met specifieke bestandsextensies toegevoegd en uitvoer in JSON-formaat
- `kiterunner brute pad/naar/doelen.txt -w pad/naar/woordenlijst.txt -x 5 -j 100` : bruteforce een lijst met doelen uit een bestand met aangepaste concurrency-instellingen voor prestaties
- `kiterunner brute https://example.com -w pad/naar/woordenlijst.txt --ignore-length 100-105` : bruteforce en negeer antwoorden met specifieke inhoudslengte
- `kiterunner brute https://example.com -w pad/naar/woordenlijst.txt -H "Authorization: Bearer token"` : bruteforce met aangepaste HTTP-headers
- `kiterunner brute pad/naar/doelen.txt -w pad/naar/woordenlijst.txt --fail-status-codes 400,401,404` : bruteforce een lijst van doelen uit een bestand met fail status code filtering

## Links
- More information: https://github.com/assetnote/kiterunner#usage
