# locate

Zum schnellen Finden von Dateinamen.

## Usage
- `locate muster` : suche nach Dateien in der Datenbank. Hinweis: Die Datenbank wird periodisch aktualisiert (für gewöhnlich täglich oder wöchentlich)
- `locate '*/dateiname'` : suche nach Dateien mit dem exakten Dateinamen. (Ein Muster ohne Platzhalterzeichen wird als `*muster*` interpretiert)
- `sudo updatedb` : aktualisiere die Datenbank. Dies ist nötig, falls kürzlich hinzugefügte Dateien gefunden werden sollen

## Links
- More information: https://manned.org/locate
