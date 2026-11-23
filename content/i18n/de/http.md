# http

HTTPie: ein benutzerfreundliches HTTP-Tool.

## Usage
- `http https://example.com` : sende eine GET-Anfrage (Zeigt die Header und den Body der Antwort)
- `http -p H|B|h|b|m|Hh|Hhb|... https://example.com` : zeige nur den angegebenen Teil der Anfrage und der Antwort (`H`: Header der Anfrage, `B`: Body der Anfrage, `h`: Header der Antwort, `b`: Body der Antwort, `m`: Metadaten der Antwort)
- `http GET|POST|HEAD|PUT|PATCH|DELETE|... --proxy http|https:http://localhost:8080|socks5://localhost:9050|... https://example.com` : spezifiziere die zu nutzende HTTP-Methode und nutze den angegebenen Proxy
- `http -F https://example.com 'User-Agent: Mozilla/5.0' 'Accept-Encoding: gzip'` : folge `3xx`-Umleitungen und spezifiziere zusätzliche Header für die Anfrage
- `http -a username:password|token -A basic|digest|bearer GET|POST|... https://example.com/auth` : authentisiere gegenüber einem Server mithilfe unterschiedlicher Anthentisierungsmethoden
- `http --offline GET|DELETE|... https://example.com` : erstelle eine Anfrage ohne diese zu senden (ähnlich zu dry-run)
- `http --session session_name|path/to/session.json -a username:password https://example.com/auth API-KEY:xxx` : nutze die angegebene Session für persistente benutzerdefinierte Header, Credentials für die Authentisierung und Cookies
- `http -f POST https://example.com/upload cv@path/to/file` : lade eine Datei in ein Formular hoch (das folgende Beispiel geht davon aus, dass das Formularfeld als `<input type="file" name="cv" />` definiert ist)

## Links
- More information: https://httpie.io/docs/cli/usage
