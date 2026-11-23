# aptitude

Debian und Ubuntu Paket Management Tool.

## Usage
- `sudo aptitude update` : synchronisiere die Paketliste und verfügbaren Versionen. Dieser Command sollte zuerst ausgeführt werden bevor weitere aptitude Commands ausgeführt werden
- `sudo aptitude install paket` : installiere ein neues Paket und seine Abhängigkeiten
- `aptitude search paket` : suche nach einem Paket
- `aptitude search '?installed (paket)'` : suche nach einem installierten Paket (`?installed` ist ein aptitude Suchbegriff)
- `sudo aptitude remove paket` : entferne ein Paket und alle Abhängigkeiten
- `sudo aptitude upgrade` : aktualisiere installierte Pakete auf die neusten Versionen
- `sudo aptitude full-upgrade` : aktualisiere installierte Pakete (wie `aptitude upgrade`), inklusive obsoleter Pakete und installiere zusätzliche Pakete um die neuen Paket-Abhängigkeiten zu erfüllen
- `sudo aptitude hold '?installed(paket)'` : friere ein installiertes Paket ein und verhindere, dass es automatisch aktualisiert wird

## Links
- More information: https://manned.org/aptitude
