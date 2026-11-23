# http

HTTPie: een HTTP-client ontworpen voor het testen, debuggen en in het algemeen interactie met API's en HTTP-servers.

## Usage
- `http https://example.com` : maak een eenvoudige GET-aanvraag (toont response headers en inhoud)
- `http -p H|B|h|b|m|Hh|Hhb|... https://example.com` : print specifieke delen van de inhoud (`H`: request headers, `B`: request body, `h`: response headers, `b`: response body, `m`: response metadata)
- `http GET|POST|HEAD|PUT|PATCH|DELETE|... --proxy http|https:http://localhost:8080|socks5://localhost:9050|... https://example.com` : specificeer de HTTP-methode bij het verzenden van een aanvraag en gebruik een proxy om de aanvraag te onderscheppen
- `http -F https://example.com 'User-Agent: Mozilla/5.0' 'Accept-Encoding: gzip'` : volg eventuele `3xx` redirects en specificeer extra headers in een verzoek
- `http -a gebruikersnaam:wachtwoord|token -A basic|digest|bearer GET|POST|... https://example.com/auth` : authenticeer bij een server met verschillende authenticatiemethoden
- `http --offline GET|DELETE|... https://example.com` : simuleer het versturen van een verzoek zonder het daadwerkelijk te versturen
- `http --session session_naam|pad/naar/session.json -a gebruikersnaam:wachtwoord https://example.com/auth API-KEY:xxx` : gebruik benoemde sessies voor aanhoudende aangepaste headers, auth-referenties en cookies
- `http -f POST https://example.com/upload cv@pad/naar/bestand` : upload een bestand naar een formulier (het onderstaande voorbeeld gaat ervan uit dat het formulier `<input type="file" name="cv" />` is)

## Links
- More information: https://httpie.io/docs/cli/usage
