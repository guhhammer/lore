# choco install

Installiere ein oder mehrere Pakete mit Chocolatey.

## Usage
- `choco install paket1 paket2 ...` : installiere ein oder mehrere Pakete, deren Namen mit Leerzeichen getrennt sind
- `choco install pfad/zu/pakete.config` : installiere Pakete aus einer Konfigurationsdatei
- `choco install pfad/zu/datei` : installiere Pakete aus einer `.nuspec`- oder `.nupkg`-Datei
- `choco install paket --version version` : installiere eine bestimmte Version eines Pakets
- `choco install paket --allow-multiple` : erlaube die gleichzeitige Installation mehrerer Versionen eines Pakets
- `choco install paket -y` : stimme allen Fragen automatisch zu
- `choco install paket -s quell_url|alias` : gib eine eigene Quelle an, von der Paket-Informationen abgerufen werden
- `choco install paket -u benutzername -p passwort` : gib einen Benutzernamen und ein Passwort für die Authentifizierung an

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
