# wcurl

Een eenvoudige wrapper rond `curl` om eenvoudig bestanden te downloaden.

## Usage
- `wcurl https://example.com/index.html` : download de inhoud van een URL naar een bestand dat wordt aangegeven door de URL (`index.html` in dit geval)
- `wcurl -o pad/naar/bestand https://example.com/index.html` : download de inhoud van een URL naar een bestand met een opgegeven naam
- `wcurl --curl-options "--progress-bar --http2" https://example.com/index.html` : download de inhoud van een URL, schakel de voortgangsbalk in en gebruik standaard HTTP/2
- `wcurl --curl-options "--clobber --continue-at -" https://example.com/index.html` : hervat een onderbroken download

## Links
- More information: https://curl.se/wcurl/manual.html
