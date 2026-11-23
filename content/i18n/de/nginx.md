# nginx

`nginx` Webserver.

## Usage
- `nginx` : starte den Server mit der standardmäßigen Konfigurationsdatei
- `nginx -c konfigurationsdatei` : starte den Server mit einer benutzerdefinierten Konfigurationsdatei
- `nginx -c konfigurationsdatei -p präfix/für/relative/pfade` : starte den Server mit einem Präfix für alle relativen Pfaden in der Konfigurationsdatei
- `nginx -t` : teste die Konfigurationsdatei ohne den laufenden Server zu beeinflussen
- `nginx -s reload` : lade die Konfigurationsdatei durch das Senden eines Signales ohne Pause neu

## Links
- More information: https://nginx.org/docs/switches.html
