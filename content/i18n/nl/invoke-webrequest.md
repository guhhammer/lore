# invoke-webrequest

Voer een HTTP/HTTPS request uit naar het Web. Opmerking: dit commando kan alleen gebruikt worden via PowerShell.

## Usage
- `Invoke-WebRequest http://example.com -OutFile pad\naar\bestand` : download de inhoud van een URL naar een bestand
- `Invoke-WebRequest http://example.com -UseBasicParsing` : geef alleen de ruwe HTML-data in plaats van deze te parsen onder Internet Explorer (alleen voor PowerShell 3.0-5.1)
- `Invoke-WebRequest -Method Post -Body @{ name='bob' } http://example.com/form` : stuur form-gecodeerde gegevens (POST request van het type `application/x-www-form-urlencoded`)
- `Invoke-WebRequest -Headers @{ X-My-Header = '123' } -Method PUT http://example.com` : stuur een request met een extra header, door gebruik te maken van een aangepaste HTTP-methode
- `Invoke-WebRequest -Body '{"name":"bob"}' -ContentType 'application/json' http://example.com/users/1234` : stuur gegevens in JSON formaat en specificeer de juiste content-type header
- `Invoke-WebRequest -Headers @{ Authorization = "Basic "+ [System.Convert]::ToBase64String([System.Text.Encoding]::ASCII.GetBytes("myusername:mypassword")) } http://example.com` : stuur een gebruikersnaam en wachtwoord voor serverauthenticatie
