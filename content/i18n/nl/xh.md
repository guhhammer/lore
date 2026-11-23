# xh

Vriendelijke en snelle tool voor het versturen van HTTP-verzoeken.

## Usage
- `xh https://postman-echo.com/get` : verstuur een GET-verzoek (toont response headers en inhoud)
- `xh post https://postman-echo.com/post name=john age=25` : verstuur een POST-verzoek met een JSON-body (sleutel-waardeparen worden toegevoegd aan een JSON-object op het hoogste niveau - bijv. `{"name": "john", "age": 25}`)
- `xh get https://postman-echo.com/response-headers foo1==bar1 foo2==bar2` : verstuur een GET-verzoek met queryparameters (bijv. <https://postman-echo.com/response-headers?foo1=bar1&foo2=bar2>)
- `xh get https://postman-echo.com header-name:header-value` : verstuur een GET-verzoek met een aangepaste header
- `xh -d https://example.com -o pad/naar/bestand` : doe een GET-verzoek en sla de responsbody op in een bestand
- `xh --offline get|delete|... https://example.com` : simuleer het versturen van een verzoek zonder het daadwerkelijk te versturen
- `xh --curl|curl-long --follow --verbose get https://example.com user-agent:curl` : toon het equivalente `curl`-commando (dit verstuurt geen enkel verzoek)

## Links
- More information: https://github.com/ducaale/xh#usage
