# ldconfig

Symbolische Verknüpfungen und Zwischenspeicher für Abhängigkeiten von gemeinsam genutzten Bibliotheken konfigurieren.

## Usage
- `sudo ldconfig` : aktualisiere symbolische Verknüpfungen und erstelle den Zwischenspeicher neu (wird normalerweise ausgeführt, wenn eine neue Bibliothek installiert wird)
- `sudo ldconfig -n pfad/zu/verzeichnis` : aktualisiere die symbolischen Verknüpfungen für ein bestimmtes Verzeichnis
- `ldconfig -p | grep bibliotheksname` : gib die Bibliotheken im Zwischenspeicher aus und prüfe ob eine bestimmte Bibliothek vorhanden ist

## Links
- More information: https://manned.org/ldconfig
