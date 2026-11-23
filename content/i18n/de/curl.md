# curl

Überträgt Daten von oder zu einem Server. Unterstützt die meisten Protokolle, inklusive HTTP, FTP und POP3.

## Usage
- `curl http://example.com -o pfad/zu/datei` : lade den Inhalt einer URL in eine Datei
- `curl -O http://example.com/datei` : lade eine Datei von einer URL herunter
- `curl -f -O -L -C - http://example.com/datei` : lade eine Datei herunter, folge Weiterleitungen und setze vergangene Dateitransfers automatisch fort
- `curl -d 'name=karl-dieter' http://example.com/formular` : sende formular-codierte Daten (POST Anfragen des Typs `application/x-www-form-urlencoded`). Benutze `--data @dateiname` oder `--data @'-'`, um von `stdin` zu lesen
- `curl -H 'X-Mein-Header: 123' -X PUT http://example.com` : sende eine Anfrage mit einem extra Header mit einer eigenen HTTP-Methode
- `curl -d '{"name":"karl-dieter"}' -H 'Content-Type: application/json' http://example.com/benutzer/1234` : sende Daten im JSON-Format und lege den geeigneten Inhaltstyp-Header fest
- `curl -u benutzername http://example.com` : übergib einen Benutzernamen und frage nach einem Passwort für die Server-Authentifizierung
- `curl -E client.pem --key key.pem -k https://example.com` : übergib Client-Zertifikat und -Schlüssel für eine Ressource und überspringe die Zertifikatsüberprüfung

## Links
- More information: https://curl.se/docs/manpage.html
