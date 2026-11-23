# curl

Zet gegevens over van of naar een server. Ondersteunt de meeste protocollen, waaronder HTTP, HTTPS, FTP, SCP, enz.

## Usage
- `curl https://example.com` : maak een HTTP GET-verzoek en dump de inhoud naar `stdout`
- `curl -L -D - https://example.com` : maak een HTTP GET-verzoek, voLg eventuele `3xx` redirects, en Dump de antwoordheaders en inhoud naar `stdout`
- `curl -O https://example.com/filename.zip` : download een bestand en sla de uitvoer op onder de bestandsnaam zoals aangegeven door de URL
- `curl -X POST -d 'name=bob' http://example.com/form` : stuur form-encoded gegevens (POST-verzoek van het type `application/x-www-form-urlencoded`). Gebruik `--data @file_name` of `--data @'-'` om van `stdin` te lezen
- `curl -k -x http://127.0.0.1:8080 -H 'Authorization: Bearer token' -X GET|PUT|POST|DELETE|PATCH|... https://example.com` : stuur een verzoek met een extra header, met behulp van een aangepaste HTTP-methode en via een proxy (zoals BurpSuite), waarbij onveilige zelfondertekende certificaten worden genegeerd
- `curl -d '{"name":"bob"}' -H 'Content-Type: application/json' http://example.com/users/1234` : verstuur gegevens in JSON-formaat, met de juiste Content-Type Header
- `curl -E client.pem --key key.pem -k https://example.com` : verstrek een clientcertificaat en privésleutel voor het verzoek, en sla de certificaatvalidatie over
- `curl -v --resolve example.com:80:127.0.0.1 http://example.com` : los een hostnaam op naar een aangepast IP-adres, met verbose uitvoer (vergelijkbaar met het bewerken van het `/etc/hosts`-bestand voor aangepaste DNS-resolutie)

## Links
- More information: https://curl.se/docs/manpage.html
