# choco upgrade

Aktualisiere mit Chocolatey ein oder mehrere Pakete.

## Usage
- `choco upgrade paket(e)` : aktualisiere ein oder mehrere Pakete, deren Namen mit Leerzeichen getrennt sind
- `choco upgrade paket --version version` : aktualisiere auf eine bestimmte Version des Pakets
- `choco upgrade all` : aktualisiere alle Pakete
- `choco upgrade all --except "paket(e)"` : aktualisiere alle außer den angegebenen, durch Kommas getrennten Paketen
- `choco upgrade paket -y` : stimme allen Fragen automatisch zu
- `choco upgrade paket -s quell_url|alias` : gib eine eigene Quelle an, von der Pakete aktualisiert werden
- `choco upgrade paket -u benutzername -p passwort` : gib einen Benutzernamen und ein Passwort für die Authentifizierung an

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/upgrade/
